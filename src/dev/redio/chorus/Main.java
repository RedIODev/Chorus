package dev.redio.chorus;

import java.io.File;
import java.io.IOException;
import java.nio.file.Path;
import java.util.ArrayList;

import dev.redio.chorus.tokenizer.token.Tokenizer;

public class Main {
    public static void main(String[] args) throws IOException {

        var tokens = Tokenizer.tokenize(Path.of("Testfile.ch"));
        for (var token : tokens) {
            System.out.println(token);
        }
        var files = new ArrayList<File>();
        for (var arg : args) {
            var path = Path.of(arg);
            var file = path.toFile();
            if (!file.exists())
                throw new IllegalArgumentException("File does not exist: %s".formatted(file.getAbsolutePath()));
            if (!file.canRead())
                throw new IllegalArgumentException("File is not readable: %s".formatted(file.getAbsolutePath()));
            files.add(file);
        }
        // Parser.processFiles(files);
    }
}