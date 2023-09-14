package dev.redio.chorus.tokenizer.token;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;


public class Tokenizer {


    public static Token[] tokenize2(String[] lines) {
        var result = new ArrayList<Token>();

        for (int i = 0; i < lines.length; i++) 
            result.addAll(processLine(lines[i], i));

        return result.toArray(Token[]::new);
    }


    private static List<Token> processLine(String line, int lineNumber) {
        var result = new ArrayList<Token>();
        final int lineLenght = line.length();
        int columnNumber = 0;
        while (columnNumber < lineLenght) {
            final char firstChar = line.charAt(columnNumber);
            if (Character.isWhitespace(firstChar)) {
                columnNumber++;
                continue;
            }

            var optKeyword = checkKeyword(line, lineNumber, columnNumber);
            if (optKeyword.isPresent()) {
                final var keyword = optKeyword.orElseThrow();
                result.add(keyword);
                columnNumber += keyword.raw().length();
                continue;
            }
            
            var optIdentifier = checkIdentifier(line, lineNumber, columnNumber);
            if (optIdentifier.isPresent()) {
                final var identifier = optIdentifier.orElseThrow();
                result.add(identifier);
                columnNumber += identifier.raw().length();
                continue;
            }
            
            throw new IllegalStateException("Cannot tokenize Ln:"+ (lineNumber+1) + "Col:" + (columnNumber+1) + "Line[" + line + "]");
        }

        return result;
    }

    private static Optional<Token> checkKeyword(String line, int lineNumber, int columnNumber) {
        final int lineLenght = line.length();
        for (var keyword : Keyword.SORTED_KEYWORDS) {
            final String raw = keyword.raw();
            final int rawLength = raw.length();
            if (rawLength > lineLenght - columnNumber)
                continue;
            var subSequence = line.subSequence(columnNumber, columnNumber + rawLength);
            if (CharSequence.compare(raw, subSequence) == 0) 
                return Optional.of(new KeywordToken(keyword,lineNumber, columnNumber));
        }
        return Optional.empty();
    }

    private static Optional<Token> checkIdentifier(String line, int lineNumber, int columnNumber) {
        final int lineLenght = line.length();
        for (int i = columnNumber; i < lineLenght; i++) {
            final char character = line.charAt(i);
            if (Character.isAlphabetic(character))
                continue;
            if (Character.isDigit(character))
                continue;
            if (Character.isWhitespace(character)) {
                var rawIdentifier = line.substring(columnNumber, i);
                return Optional.of(new Identifier(rawIdentifier, lineNumber, columnNumber));
            }
            for (var keyword : Keyword.SYMBOLIC_KEYWORDS) {
                final String raw = keyword.raw();
                final int rawLength = raw.length();
                if (rawLength > lineLenght - i)
                    continue;
                var subSequence = line.subSequence(i, i + rawLength);
                if (CharSequence.compare(raw, subSequence) == 0) {
                    var rawIdentifier = line.substring(columnNumber, i);
                    return Optional.of(new Identifier(rawIdentifier,lineNumber, columnNumber));
                }  
            }
        }

        return Optional.empty();
    }



    // public static Token[] tokenize(Path path) throws IOException {
    //     var result = new ArrayList<Token>();

    //     var lines = Files.lines(path).toArray(String[]::new);
    //     for (int i = 0; i < lines.length; i++) {
    //         final String line = lines[i];
    //         int nextParsedIndex = 0;
    //         CharSequence trimedSequence = line;
    //         while(nextParsedIndex < line.length()) {
    //             trimedSequence = FormatHelper.trim(trimedSequence.subSequence(nextParsedIndex, trimedSequence.length()));
    //             if (trimedSequence.isEmpty())
    //                 break;
    //             var tokenResult = findToken(trimedSequence, i);
    //             nextParsedIndex = tokenResult.nextParsedIndex();
    //             result.add(tokenResult.token());
    //         }
            
    //     }
    //     return result.toArray(Token[]::new);
    // }

    // private static FindTokenResult findToken(CharSequence lineSubSequence, int currentLine) {
    //     for (var keyword : Keyword.SORTED_KEYWORDS) {
    //         var opt = checkSequenceForKeyword(keyword, lineSubSequence, currentLine);
    //         if (opt.isEmpty())
    //             continue;
    //         return opt.get();
    //     }
        
    //     return findIdentifier(lineSubSequence, currentLine);

    // }

    // private static FindTokenResult findIdentifier(CharSequence lineSubSequence, int currentLine) {

    //     for (int i = 0; i < lineSubSequence.length(); i++) {
    //         if (Character.isAlphabetic(lineSubSequence.charAt(i)))
    //             continue;
    //         if (Character.isDigit(lineSubSequence.charAt(i)))
    //             continue;
    //         if (Character.isWhitespace(lineSubSequence.charAt(i))) {
    //             return new FindTokenResult(i, new Identifier(lineSubSequence.subSequence(0, i).toString(), currentLine));
    //         }
    //         var endSequence = lineSubSequence.subSequence(i, lineSubSequence.length());

    //         for (var token : Keyword.SYMBOLIC_KEYWORDS) {
    //             var opt = checkSequenceForKeyword(token, endSequence, currentLine);
    //             if (opt.isEmpty())
    //                 continue;
                
    //             return new FindTokenResult(i, new Identifier(lineSubSequence.subSequence(0, i).toString(), currentLine));
    //         }
    //     }
    //     return new FindTokenResult(lineSubSequence.length(), new Identifier(lineSubSequence.toString(), currentLine));
    // }

    // private static Optional<FindTokenResult> checkSequenceForKeyword(Keyword keyword, CharSequence sequence, int currentLine) {
    //     final var rawTokenString = keyword.raw();
    //         if (sequence.length() < rawTokenString.length())
    //             return Optional.empty();
    //         var tokenSubSequence = sequence.subSequence(0, rawTokenString.length());

    //         if (CharSequence.compare(tokenSubSequence, rawTokenString) != 0)
    //             return Optional.empty();
    //         return Optional.of(new FindTokenResult(rawTokenString.length(), new KeywordToken(keyword, currentLine)));
    // }

    // private record FindTokenResult(int nextParsedIndex, Token token) {}
}

