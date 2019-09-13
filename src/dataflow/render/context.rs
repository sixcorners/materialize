// Copyright 2019 Materialize, Inc. All rights reserved.
//
// This file is part of Materialize. Materialize may not be used or
// distributed without the express permission of Materialize, Inc.

//! Management of arrangements while building a dataflow.

use std::collections::HashMap;

use timely::dataflow::{Scope, ScopeParent};
use timely::progress::{timestamp::Refines, Timestamp};

use differential_dataflow::lattice::Lattice;
use differential_dataflow::operators::arrange::{Arranged, TraceAgent};
use differential_dataflow::trace::implementations::ord::OrdValSpine;
use differential_dataflow::trace::wrappers::enter::TraceEnter;
use differential_dataflow::trace::wrappers::frontier::TraceFrontier;
use differential_dataflow::Collection;
use differential_dataflow::Data;

/// A trace handle for key-value data.
pub type TraceValHandle<K, V, T, R> = TraceAgent<OrdValSpine<K, V, T, R>>;

type Diff = isize;

// Local type definition to avoid the horror in signatures.
type Arrangement<S, V> =
    Arranged<S, TraceValHandle<Vec<V>, Vec<V>, <S as ScopeParent>::Timestamp, Diff>>;
type ArrangementImport<S, V, T> = Arranged<
    S,
    TraceEnter<
        TraceFrontier<TraceValHandle<Vec<V>, Vec<V>, T, Diff>>,
        <S as ScopeParent>::Timestamp,
    >,
>;

/// Dataflow-local collections and arrangements.
///
/// A context means to wrap available data assets and present them in an easy-to-use manner.
/// These assets include dataflow-local collections and arrangements, as well as imported
/// arrangements from outside the dataflow.
///
/// Context has two timestamp types, one from `S::Timestamp` and one from `T`, where the
/// former must refine the latter. The former is the timestamp used by the scope in question,
/// and the latter is the timestamp of imported traces. The two may be different in the case
/// of regions or iteration.
pub struct Context<S: Scope, P, V: Data, T>
where
    P: Eq + std::hash::Hash + Clone,
    T: Timestamp + Lattice,
    S::Timestamp: Lattice + Refines<T>,
{
    /// Dataflow local collections.
    pub collections: HashMap<P, Collection<S, Vec<V>, Diff>>,
    /// Dataflow local arrangements.
    pub local: HashMap<P, HashMap<Vec<usize>, Arrangement<S, V>>>,
    /// Imported arrangements.
    #[allow(clippy::type_complexity)] // TODO(fms): fix or ignore lint globally.
    pub trace: HashMap<P, HashMap<Vec<usize>, ArrangementImport<S, V, T>>>,
}

impl<S: Scope, P, V: Data, T> Context<S, P, V, T>
where
    P: Eq + std::hash::Hash + Clone,
    T: Timestamp + Lattice,
    S::Timestamp: Lattice + Refines<T>,
{
    /// Creates a new empty Context.
    pub fn new() -> Self {
        Self {
            collections: HashMap::new(),
            local: HashMap::new(),
            trace: HashMap::new(),
        }
    }

    /// Indicates if a collection is available.
    pub fn has_collection(&self, relation_expr: &P) -> bool {
        self.collections.get(relation_expr).is_some()
            || self.local.get(relation_expr).is_some()
            || self.trace.get(relation_expr).is_some()
    }

    /// Assembles a collection if available.
    ///
    /// This method consults all available data assets to create the appropriate
    /// collection. This can be either a collection itself, or if absent we may
    /// also be able to find a stashed arrangement for the same relation_expr, which we
    /// flatten down to a collection.
    ///
    /// If insufficient data assets exist to create the collection the method
    /// will return `None`.
    pub fn collection(&self, relation_expr: &P) -> Option<Collection<S, Vec<V>, Diff>> {
        if let Some(collection) = self.collections.get(relation_expr) {
            Some(collection.clone())
        } else if let Some(local) = self.local.get(relation_expr) {
            Some(
                local
                    .values()
                    .next()
                    .expect("Empty arrangement")
                    .as_collection(|_k, v| v.clone()),
            )
        } else if let Some(trace) = self.trace.get(relation_expr) {
            Some(
                trace
                    .values()
                    .next()
                    .expect("Empty arrangement")
                    .as_collection(|_k, v| v.clone()),
            )
        } else {
            None
        }
    }

    /// Produces an arrangement if available.
    ///
    /// A context store multiple types of arrangements, and prioritizes
    /// dataflow-local arrangements in its return values.
    pub fn arrangement(
        &self,
        relation_expr: &P,
        keys: &[usize],
    ) -> Option<ArrangementFlavor<S, V, T>> {
        if let Some(local) = self.get_local(relation_expr, keys) {
            Some(ArrangementFlavor::Local(local.clone()))
        } else if let Some(trace) = self.get_trace(relation_expr, keys) {
            Some(ArrangementFlavor::Trace(trace.clone()))
        } else {
            None
        }
    }

    /// Retrieves a local arrangement from a relation_expr and keys.
    pub fn get_local(&self, relation_expr: &P, keys: &[usize]) -> Option<&Arrangement<S, V>> {
        self.local.get(relation_expr).and_then(|x| x.get(keys))
    }

    /// Binds a relation_expr and keys to a local arrangement.
    pub fn set_local(&mut self, relation_expr: &P, keys: &[usize], arranged: Arrangement<S, V>) {
        self.local
            .entry(relation_expr.clone())
            .or_insert_with(|| HashMap::new())
            .insert(keys.to_vec(), arranged);
    }

    /// Retrieves an imported arrangement from a relation_expr and keys.
    pub fn get_trace(
        &self,
        relation_expr: &P,
        keys: &[usize],
    ) -> Option<&ArrangementImport<S, V, T>> {
        self.trace.get(relation_expr).and_then(|x| x.get(keys))
    }

    /// Binds a relation_expr and keys to an imported arrangement.
    #[allow(dead_code)]
    pub fn set_trace(
        &mut self,
        relation_expr: &P,
        keys: &[usize],
        arranged: ArrangementImport<S, V, T>,
    ) {
        self.trace
            .entry(relation_expr.clone())
            .or_insert_with(|| HashMap::new())
            .insert(keys.to_vec(), arranged);
    }

    /// Clones from one key to another, as needed in let binding.
    pub fn clone_from_to(&mut self, key1: &P, key2: &P) {
        if let Some(collection) = self.collections.get(key1).cloned() {
            self.collections.insert(key2.clone(), collection);
        }
        if let Some(handles) = self.local.get(key1).cloned() {
            self.local.insert(key2.clone(), handles);
        }
        if let Some(handles) = self.trace.get(key1).cloned() {
            self.trace.insert(key2.clone(), handles);
        }
    }
}

/// Describes flavor of arrangement: local or imported trace.
pub enum ArrangementFlavor<S: Scope, V: Data, T: Lattice>
where
    T: Timestamp + Lattice,
    S::Timestamp: Lattice + Refines<T>,
{
    /// A dataflow-local arrangement.
    Local(Arrangement<S, V>),
    /// An imported trace from outside the dataflow.
    Trace(ArrangementImport<S, V, T>),
}