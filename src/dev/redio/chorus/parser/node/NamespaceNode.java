package dev.redio.chorus.parser.node;

import java.util.Optional;

import dev.redio.chorus.parser.EmptyArrays;
import dev.redio.chorus.parser.exception.IllegalChildException;
import dev.redio.chorus.parser.exception.IllegalParentException;

public class NamespaceNode implements ContainerNode {

    private final Node parent;
    private final AccessModifier accessModifier;
    private final IdentifierNode identifier;
    private Node[] childs = EmptyArrays.NODES_EMPTY;

    public NamespaceNode(Node parent, AccessModifier accessModifier, IdentifierNode identifier) {
        this.parent = switch (parent) {
            case NamespaceNode nt -> nt;
            case FileNode ft -> ft;
            case null -> throw new IllegalParentException(parent, this);
            default -> throw new IllegalParentException(parent, this);
        };

        if (accessModifier == null) 
            throw new IllegalArgumentException("accessModifier cannot be null");

        this.accessModifier = accessModifier;

        if (identifier == null)
            throw new IllegalArgumentException("identifier cannot be null");

        this.identifier = identifier;
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
        for (Node child : childs) {
            switch (child) {
                case NamespaceNode nt -> {}

                case null -> throw new IllegalChildException(child, this);
                default -> throw new IllegalChildException(child, this);
            }
        }
        this.childs = childs;
    }

    public AccessModifier accessModifier() {
        return accessModifier;
    }

    public IdentifierNode identifier() {
        return identifier;
    }

    @Override
    public String toString() {
        var builder = new StringBuilder("Namespace[accessModifier=");
        builder.append(accessModifier.toString());
        builder.append(", identifier=");
        builder.append(identifier.toString());
        builder.append(", childs=[");
        for (var child : childs) {
            builder.append(child.toString());
            builder.append(", ");
        }
        builder.setLength(builder.length()-2);
        builder.append("]]");
        return builder.toString();
    }

    @Override
    public String raw() {
        var builder = new StringBuilder(accessModifier.raw());
        builder.append(" namespace ");
        builder.append(identifier.raw());
        builder.append(" {");
        builder.append(System.lineSeparator());
        for (var child : childs) {
            builder.append(child.raw().indent(4));
            builder.append(System.lineSeparator());
        }
        builder.append("}");
        return builder.toString();
    }

    
    
}
