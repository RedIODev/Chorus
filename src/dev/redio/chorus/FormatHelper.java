package dev.redio.chorus;

public final class FormatHelper {
    private FormatHelper() {}

    public static String indent(String input) {
        var lines = input.split(System.lineSeparator());
        StringBuilder builder = new StringBuilder();
        for (var line : lines) {
            builder.append("\t");
            builder.append(line);
            builder.append(System.lineSeparator());
        }
        builder.setLength(builder.length()- System.lineSeparator().length());
        return builder.toString();
    }

    public static CharSequence trim(CharSequence source) {
        int startIndex = 0;
        for (int i = 0; i < source.length(); i++) {
            if (Character.isWhitespace(source.charAt(i))) 
                continue;
            startIndex = i;
            break;
        }
        int endIndexInclusive = source.length() - 1;
        for (int i = source.length()- 1; i >= 0; i--) {
            if (Character.isWhitespace(source.charAt(i)))
                continue;
            endIndexInclusive = i;
            break;
        }
        return source.subSequence(startIndex, endIndexInclusive + 1);
    }

    // public static CharSequence[] charSequenceSplit(CharSequence input, CharSequence seperator) {
    //     final char firstSeperator = seperator.charAt(0);
    //     var result = new ArrayList<CharSequence>();
    //     int lastEndIndex = 0;
    //     for (int i = 0; i < input.length(); i++) {
    //         if (input.charAt(i) != firstSeperator)
    //             continue;
    //         if (i + seperator.length() > input.length()) {
    //             result.add(input.subSequence(lastEndIndex, input.length()));
    //             break;
    //         }
    //         if (CharSequence.compare(input.subSequence(i, i + seperator.length()),seperator) != 0)
    //             continue;
    //         result.add(input.subSequence(lastEndIndex, i));
    //         lastEndIndex = i + seperator.length();
    //     }
    //     return result.toArray(CharSequence[]::new);
    // }
}
