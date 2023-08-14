package dev.redio.chorus.parser.node;

import java.nio.file.Path;
import java.util.Optional;

import dev.redio.chorus.parser.exception.IllegalChildException;

public class FileNode implements ContainerNode {

    private Node[] childs = ContainerNode.EMPTY;
    private final Path sourcePath;

    public FileNode(Path sourcePath) {
        if (sourcePath == null)
            throw new IllegalArgumentException("sourcePath cannot be null");
        this.sourcePath = sourcePath;
    }

    @Override
    public Optional<Node> parent() {
       return Optional.empty();
    }

    @Override
    public Node[] childs() {
        return childs;
    }

     @Override
    public void setChilds(Node[] childs) {
        for (var child : childs) {
            switch (child) {
                case NamespaceNode nn -> {}
                case null -> throw new IllegalChildException(child, this);
                default -> throw new IllegalChildException(child, this);
            }
        }
        this.childs = childs;
    }

    public Path sourcePath() {
        return sourcePath;
    }

    @Override
    public String toString() {
        var builder = new StringBuilder("File[sourcePath=");
        builder.append(sourcePath.toString());
        builder.append(", childs[");
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
        for (var child : childs) {
            builder.append(child.raw());
            builder.append(System.lineSeparator());
        }
        builder.setLength(builder.length()- System.lineSeparator().length());
        return builder.toString();
    }

   

    
}
