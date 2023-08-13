package dev.redio.chorus.parser.node;

import java.util.Optional;

import dev.redio.chorus.parser.exception.IllegalParentException;

public class PathNode implements Node {

    private final Node parent;

    private final IdentifierNode[] path;

    private static final IdentifierNode[] EMPTY = {};

    public PathNode(Node parent, IdentifierNode... path) {
        this.parent = switch (parent) {
            case ImportNode in -> in;
            case null -> throw new IllegalParentException(parent);
            default -> throw new IllegalParentException(parent);
        };

        if (path == null) {
            this.path = EMPTY;
            return;
        }

        this.path = path;
    }


    @Override
    public Optional<Node> parent() {
        return Optional.of(parent);
    }

    public IdentifierNode[] path() {
        return path;
    }

    @Override 
    public String toString()  {
        var result = new StringBuilder("Path[");
        for (var ident : path) {
            result.append(ident.toString());
            result.append(", ");
        }

        result.setLength(result.length()-2);
        result.append("]");
        return result.toString();
    }


    @Override
    public String raw() {
        var builder = new StringBuilder("import ");
        for (var element : path) {
            builder.append(element.raw());
            builder.append("::");
        }
        builder.setLength(builder.length()-2);
        return builder.toString();
    }
    
}
