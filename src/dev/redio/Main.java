package dev.redio;

import java.io.File;
import java.nio.file.Path;
import java.util.ArrayList;

import dev.redio.chorus.tokenizer.FormatHelper;

public class Main {
    public static void main(String[] args) {
        var s = """
                    Test
                    Multi
                    Line""";
        var s2 = FormatHelper.indent(s);
        System.out.println(s);
        System.out.println(s2);
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
        //Parser.processFiles(files);
    }
}