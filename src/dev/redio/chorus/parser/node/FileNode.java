package dev.redio.chorus.parser.node;

import java.nio.file.Path;
import java.util.Optional;

public class FileNode implements ContainerNode {

    private final NamespaceNode[] childs;
    private final Path sourcePath;

    private static final NamespaceNode[] EMPTY = {};

    public FileNode(Path sourcePath, NamespaceNode... childs) {
        if (sourcePath == null)
            throw new IllegalArgumentException("sourcePath cannot be null");
        this.sourcePath = sourcePath;
        if (childs == null) 
            childs = EMPTY;
        this.childs = childs;
    }

    @Override
    public Optional<Node> parent() {
       return Optional.empty();
    }

    @Override
    public NamespaceNode[] childs() {
        return childs;
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
