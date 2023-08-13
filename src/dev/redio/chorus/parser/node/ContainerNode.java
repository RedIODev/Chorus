package dev.redio.chorus.parser.node;

public interface ContainerNode extends Node {
    Node[] childs();
    static Node[] EMPTY = {};
}
