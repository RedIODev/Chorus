package dev.redio.chorus.parser.node;

import java.util.Optional;

import dev.redio.chorus.parser.exception.IllegalChildException;
import dev.redio.chorus.parser.exception.IllegalParentException;

public class ImportNode implements Node {

    private final Node parent;

    private final PathNode path;
    
    private final IdentifierNode alias;

    public ImportNode(Node parent, PathNode path) {
        this(parent, path, null);
    }

    public ImportNode(Node parent, PathNode path, IdentifierNode alias) {
        this.parent = switch (parent) {
            case NamespaceNode nt -> nt;
            case null -> throw new IllegalParentException(parent, this);
            default -> throw new IllegalParentException(parent, this);
        };
        if (path == null) 
            throw new IllegalChildException(path, this);
        this.path = path;

        this.alias = alias;
    }
    @Override
    public Optional<Node> parent() {
        return Optional.of(parent);
    }

    public PathNode path() {
        return path;
    }

    public Optional<IdentifierNode> alias() {
        return Optional.ofNullable(alias);
    }
  
    @Override
    public String toString() {
        return "Import[path=" + path.toString() + ((alias == null) ? "]" : ", alias=" + alias.toString() + "]");
    }

    @Override
    public String raw() {
        var builder = new StringBuilder("import ");
        builder.append(path.raw());
        if (alias == null) {
            builder.append(" as ");
            builder.append(alias.raw());
        }
        builder.append(";");
        return builder.toString();
    }
}
