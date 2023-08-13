package dev.redio.chorus.parser.node;

import java.util.Optional;

import dev.redio.chorus.parser.exception.IllegalChildException;
import dev.redio.chorus.parser.exception.IllegalParentException;

public class NamespaceNode implements ContainerNode {

    private final Node parent;
    private final AccessModifier accessModifier;
    private final IdentifierNode identifier;
    private final Node[] childs;

    public NamespaceNode(Node parent, AccessModifier accessModifier, IdentifierNode identifier, Node... childs) {
        this.parent = switch (parent) {
            case NamespaceNode nt -> nt;
            case FileNode ft -> ft;
            case null -> throw new IllegalParentException(parent);
            default -> throw new IllegalParentException(parent);
        };

        if (accessModifier == null) 
            throw new IllegalArgumentException("accessModifier cannot be null");

        this.accessModifier = accessModifier;

        if (identifier == null)
            throw new IllegalArgumentException("identifier cannot be null");

        this.identifier = identifier;

        if (childs == null) {
            childs = ContainerNode.EMPTY;
        }
        
        for (Node child : childs) {
            switch (child) {
                case NamespaceNode nt -> {}

                case null -> throw new IllegalChildException(child);
                default -> throw new IllegalChildException(child);
            }
        }
        this.childs = childs;
    }

    @Override
    public Optional<Node> parent() {
        return Optional.of(parent);
    }

    @Override
    public Node[] childs() {
       return childs;
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
