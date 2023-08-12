package dev.redio.chorus.tokenizer;

import java.util.ArrayList;

public final class FormatHelper {
    private FormatHelper() {}

    public static String indent(String input) {
        CharSequence[] lines = charSequenceSplit(input, System.lineSeparator());
        StringBuilder builder = new StringBuilder();
        for (var line : lines) {
            builder.append("\t");
            builder.append(line);
            builder.append(System.lineSeparator());
        }
        builder.setLength(builder.length()- System.lineSeparator().length()+1);
        return builder.toString();
    }

    public static CharSequence[] charSequenceSplit(CharSequence input, CharSequence seperator) {
        final char firstSeperator = seperator.charAt(0);
        var result = new ArrayList<CharSequence>();
        int lastEndIndex = 0;
        for (int i = 0; i < input.length(); i++) {
            if (input.charAt(i) != firstSeperator) 
                continue;
            if (i + seperator.length() > input.length()) {
                result.add(input.subSequence(lastEndIndex, input.length()));
                break;
            }
            if (CharSequence.compare(input.subSequence(i, i + seperator.length()),seperator) != 0)
                continue;
            result.add(input.subSequence(lastEndIndex, i));
            lastEndIndex = i + seperator.length();
        }
        return result.toArray(CharSequence[]::new);
    }
}
