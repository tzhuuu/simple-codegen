# TODO
1. Support visibility on struct tuple fields
2. Refactor all the exposed types (for instance AssociatedConst probably needs to refactored, and so are bounds)
3. Update documentation
4. Create fuller test examples (changing values, etc.)
5. Longer-term: think about scoping the generics within scopes. For instance, functions inside an impl block should always have the same generics.
6. Validate the associated const having a value for an impl.
