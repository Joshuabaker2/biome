use crate::DeserializationDiagnostic;
use biome_rowan::{Language, SyntaxNode};

/// Generic trait to implement when resolving the configuration from a generic language
pub trait VisitNode<L: Language>: Sized {
    /// Called when visiting a value that is not a list or a map.
    /// A value can be a string, an integer, a boolean.
    fn visit_value(
        &mut self,
        _node: &SyntaxNode<L>,
        _diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        unimplemented!("you should implement visit_value")
    }

    /// Called when visiting a list of key-value.
    ///
    /// The implementor should loop through the list and call this function by passing two nodes,
    /// the key/name as first argument, and the value as second argument.
    fn visit_map(
        &mut self,
        _key: &SyntaxNode<L>,
        _value: &SyntaxNode<L>,
        _diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        unimplemented!("you should implement visit_map")
    }

    /// Called when visiting a list of elements.
    ///
    /// The implementor should loop through the list and call this function by passing the encountered nodes.
    fn visit_array_member(
        &mut self,
        _element: &SyntaxNode<L>,
        _diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        unimplemented!("you should implement visit_array_member")
    }
}
