package dev.redio.chorus.tokenizer.token;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Optional;

import dev.redio.chorus.FormatHelper;


public class Tokenizer {

    public static Token[] tokenize(Path path) throws IOException {
        var result = new ArrayList<Token>();

        var lines = Files.lines(path).toArray(String[]::new);
        for (String line : lines) {
            int nextParsedIndex = 0;
            CharSequence trimedSequence = line;
            while(nextParsedIndex < line.length()) {
                trimedSequence = FormatHelper.trim(trimedSequence.subSequence(nextParsedIndex, trimedSequence.length()));
                if (trimedSequence.isEmpty())
                    break;
                var tokenResult = findToken(trimedSequence);
                nextParsedIndex = tokenResult.nextParsedIndex();
                result.add(tokenResult.token());
            }
            
        }
        return result.toArray(Token[]::new);
    }

    private static FindTokenResult findToken(CharSequence lineSubSequence) {
        for (var keyword : Keyword.SORTED_KEYWORDS) {
            var opt = checkSequenceForKeyword(keyword, lineSubSequence);
            if (opt.isEmpty())
                continue;
            return opt.get();
        }
        
        return findIdentifier(lineSubSequence);

    }

    private static FindTokenResult findIdentifier(CharSequence lineSubSequence) {

        for (int i = 0; i < lineSubSequence.length(); i++) {
            if (Character.isAlphabetic(lineSubSequence.charAt(i)))
                continue;
            if (Character.isDigit(lineSubSequence.charAt(i)))
                continue;
            if (Character.isWhitespace(lineSubSequence.charAt(i))) {
                return new FindTokenResult(i, new Identifier(lineSubSequence.subSequence(0, i).toString()));
            }
            var endSequence = lineSubSequence.subSequence(i, lineSubSequence.length());

            for (var token : Keyword.SYMBOLIC_KEYWORDS) {
                var opt = checkSequenceForKeyword(token, endSequence);
                if (opt.isEmpty())
                    continue;
                
                return new FindTokenResult(i, new Identifier(lineSubSequence.subSequence(0, i).toString()));
            }
        }
        return new FindTokenResult(lineSubSequence.length(), new Identifier(lineSubSequence.toString()));
    }

    private static Optional<FindTokenResult> checkSequenceForKeyword(Keyword keyword, CharSequence sequence) {
        final var rawTokenString = keyword.raw();
            if (sequence.length() < rawTokenString.length())
                return Optional.empty();
            var tokenSubSequence = sequence.subSequence(0, rawTokenString.length());

            if (CharSequence.compare(tokenSubSequence, rawTokenString) != 0)
                return Optional.empty();
            return Optional.of(new FindTokenResult(rawTokenString.length(), keyword));
    }

    private record FindTokenResult(int nextParsedIndex, Token token) {}
}

