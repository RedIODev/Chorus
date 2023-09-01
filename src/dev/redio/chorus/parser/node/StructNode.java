package dev.redio.chorus.parser.node;

import java.util.Optional;

import dev.redio.chorus.parser.EmptyArrays;
import dev.redio.chorus.parser.exception.IllegalChildException;
import dev.redio.chorus.parser.exception.IllegalModifierException;
import dev.redio.chorus.parser.exception.IllegalParentException;

public class StructNode implements ContainerNode {

    private final Node parent;
    private final AccessModifier accessModifier;
    private final Modifier[] modifiers;
    private final IdentifierNode identifier;
    private final GenericParameterNode[] genericParameters;
    
    private Node[] childs = EmptyArrays.NODES_EMPTY;

    public StructNode(Node parent, AccessModifier accessModifier, Modifier[] modifiers, IdentifierNode identifier, GenericParameterNode[] genericParameters) {
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
    public Node[] childs() {
        return childs;
    }

    @Override
    public void setChilds(Node[] childs) {
        for (var child : childs) {
            switch (child) {
                case FieldNode fn -> {
                }
                case null -> throw new IllegalChildException(child, this);
                default -> throw new IllegalChildException(child, this);
            }
        }
        this.childs = childs;
    }

    public AccessModifier accessModifier() {
        return accessModifier;
    }

    public Modifier[] modifiers() {
        return modifiers;
    }

    public IdentifierNode identifier() {
        return identifier;
    }

    @Override
    public String toString() {
        var builder = new StringBuilder("Struct[accessModifier=");
        builder.append(accessModifier.toString());
        builder.append(", modifiers[");
        for (var modifier : modifiers) {
            builder.append(modifier.toString());
            builder.append(", ");
        }
        builder.setLength(builder.length() - 2);
        builder.append("], identifier=");
        builder.append(identifier.toString());
        builder.append(", genericParameters[");
        for (var parameter : genericParameters) {
            builder.append(parameter.toString());
            builder.append(", ");
        }
        builder.setLength(builder.length() - 2);
        builder.append("], childs[");
        for (var child : childs) {
            builder.append(child.toString());
            builder.append(", ");
        }
        builder.setLength(builder.length() - 2);
        builder.append("]]");
        return builder.toString();
    }

    @Override
    public String raw() {
        var builder = new StringBuilder();
        builder.append(accessModifier.raw());
        builder.append(" ");
        for (var modifier : modifiers) {
            builder.append(modifier.raw());
            builder.append(" ");
        }
        builder.append("struct ");
        builder.append(identifier.raw());
        builder.append("<");
        for (var parameter : genericParameters) {
            builder.append(parameter.raw());
            builder.append(", ");
        }
        builder.setLength(builder.length() - 2);
        builder.append("> {");
        builder.append(System.lineSeparator());
        for (var child : childs) {
            builder.append(child.raw().indent(4));
            builder.append(System.lineSeparator());
        }
        builder.append("}");
        return builder.toString();
    }

}
