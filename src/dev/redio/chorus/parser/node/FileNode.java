package dev.redio.chorus.parser.node;

import java.util.Optional;

public class FileNode implements Node {

    

    @Override
    public Optional<Node> parent() {
       return Optional.empty();
    }

    @Override
    public Node[] childs() {
        // TODO Auto-generated method stub
        throw new UnsupportedOperationException("Unimplemented method 'childs'");
    }

    
}
