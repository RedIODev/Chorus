package dev.redio.chorus.parser.node;

import java.util.Optional;

import dev.redio.chorus.parser.EmptyArrays;
import dev.redio.chorus.parser.exception.IllegalModifierException;
import dev.redio.chorus.parser.exception.IllegalParentException;

public class TupleStructNode implements Node {
    private final Node parent;
    private final AccessModifier accessModifier;
    private final Modifier[] modifiers;
    private final IdentifierNode identifier;
    private final GenericParameterNode[] genericParameters;

    private final FieldNode[] fields = {};

    public TupleStructNode(Node parent, AccessModifier accessModifier, Modifier[] modifiers, IdentifierNode identifier, GenericParameterNode[] genericParameters) {
        this.parent = switch (parent) {
            case NamespaceNode nn -> nn;
            case null -> throw new IllegalParentException(parent, this);
            default -> throw new IllegalParentException(parent, this);
        };

        if (accessModifier == null)
            throw new IllegalArgumentException("accessModifier cannot be null");

        this.accessModifier = accessModifier;

        if (identifier == null)
            throw new IllegalArgumentException("identifier cannot be null");

        this.identifier = identifier;

        if (modifiers == null)
            modifiers = EmptyArrays.MODIFIERS_EMPTY;

        for (var modifier : modifiers) {
            switch (modifier) {
                case ASYNC, STATIC, INLINE, UNSIGNED, CONST -> throw new IllegalModifierException(modifier, this);
                case EXTERN, UNSAFE -> {
                }
            }
        }
        this.modifiers = modifiers;

        if (genericParameters == null)
            genericParameters = EmptyArrays.GENERIC_PARAMETER_NODES_EMPTY;
        this.genericParameters = genericParameters;
    }


    @Override
    public Optional<Node> parent() {
        return Optional.of(parent);
    }


    @Override
    public String toString() {
        // TODO Auto-generated method stub
        throw new UnsupportedOperationException("Unimplemented method 'toString'");
    }

    @Override
    public String raw() {
        // TODO Auto-generated method stub
        throw new UnsupportedOperationException("Unimplemented method 'raw'");
    }
}
