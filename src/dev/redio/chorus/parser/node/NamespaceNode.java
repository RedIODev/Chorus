package dev.redio.chorus.parser.node;

import java.util.Optional;

import dev.redio.chorus.FormatHelper;
import dev.redio.chorus.parser.exception.IllegalChildException;
import dev.redio.chorus.parser.exception.IllegalParentException;

public class NamespaceNode implements Node {

    private final Node parent;
    private final Node[] childs;

    public NamespaceNode(Node parent, Node... childs) {
        this.parent = switch (parent) {
            case NamespaceNode nt -> nt;
            case FileNode ft -> ft;
            case null -> throw new IllegalParentException(parent);
            default -> throw new  IllegalParentException(parent);
        };
        if (childs == null) {
            this.childs = new Node[0];
            return;
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

    @Override
    public String toString() {
        StringBuilder builder = new StringBuilder("namespace {");
        for (var child : childs) {
            builder.append(FormatHelper.indent(child.toString()));
            builder.append(System.lineSeparator());
        }

        builder.append("}");
        return builder.toString();
    }
    
}
