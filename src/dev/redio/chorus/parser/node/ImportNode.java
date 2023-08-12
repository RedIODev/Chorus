package dev.redio.chorus.parser.node;

import java.util.Optional;

import dev.redio.chorus.parser.exception.IllegalParentException;

public class ImportNode implements Node {

    private final Node parent;

    public ImportNode(Node parent) {
        this.parent = switch (parent) {
            case NamespaceNode nt -> nt;
            case null -> throw new IllegalParentException(parent);
            default -> throw new IllegalParentException(parent);
        };
    }
    @Override
    public Optional<Node> parent() {
        return Optional.of(parent);
    }

    @Override
    public Node[] childs() {
        // TODO Auto-generated method stub
        throw new UnsupportedOperationException("Unimplemented method 'childs'");
    }


  
    
}
