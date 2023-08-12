package dev.redio.chorus.parser.node;

import java.util.Optional;

public interface Node {
    Optional<Node> parent();
    Node[] childs();
}
