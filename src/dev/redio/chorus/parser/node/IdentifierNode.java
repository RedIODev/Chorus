package dev.redio.chorus.parser.node;

import java.util.Optional;

import dev.redio.chorus.parser.exception.IllegalParentException;

public class IdentifierNode implements Node {

    private final Node parent;

    private final String name;

    public IdentifierNode(Node parent, String name) {
        this.parent = switch (parent) {
            case PathNode pn -> pn;
            case null -> throw new IllegalParentException(parent);
            default -> throw new IllegalParentException(parent);
        };

        if (name == null) 
            throw new IllegalArgumentException("Name cannot be null.");
        this.name = name;
    }

    @Override
    public Optional<Node> parent() {
        return Optional.of(parent);
    }

    public String name() {
        return name;
    }

    @Override
    public String toString() {
        return "Identifier[" + name + "]";
    }

    @Override
    public String raw() {
        return name;
    }
    
}
