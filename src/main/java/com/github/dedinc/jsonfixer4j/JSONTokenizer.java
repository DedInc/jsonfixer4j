package com.github.dedinc.jsonfixer4j;

import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public final class JSONTokenizer {

    private static final Pattern NUM_PATTERN = Pattern.compile("^-?\\d+(\\.\\d+)?$");
    private static final Pattern ID_PATTERN = Pattern.compile("^[A-Za-z_]\\w*$");

    public List<Token> tokenize(String s) {

        List<Token> tokens = new ArrayList<>(s.length() / 2);
        int i = 0;
        final int length = s.length();
        while (i < length) {
            char c = s.charAt(i);
            if (Character.isWhitespace(c)) {
                i++;
                continue;
            }
            switch (c) {
                case '{':
                    tokens.add(new Token(TokenKind.LBRACE, "{"));
                    i++;
                    break;
                case '}':
                    tokens.add(new Token(TokenKind.RBRACE, "}"));
                    i++;
                    break;
                case '[':
                    tokens.add(new Token(TokenKind.LBRACKET, "["));
                    i++;
                    break;
                case ']':
                    tokens.add(new Token(TokenKind.RBRACKET, "]"));
                    i++;
                    break;
                case ':':
                    tokens.add(new Token(TokenKind.COLON, ":"));
                    i++;
                    break;
                case ',':
                    tokens.add(new Token(TokenKind.COMMA, ","));
                    i++;
                    break;
                case '"': {
                    i++;
                    StringBuilder sb = new StringBuilder();
                    while (i < length) {
                        char current = s.charAt(i);
                        if (current == '"') {
                            i++;
                            break;
                        } else if (current == '\\') {
                            if (i + 1 < length) {
                                sb.append(current);
                                sb.append(s.charAt(i + 1));
                                i += 2;
                            } else {
                                i++;
                                break;
                            }
                        } else {
                            sb.append(current);
                            i++;
                        }
                    }
                    tokens.add(new Token(TokenKind.STRING, sb.toString()));
                    break;
                }
                default:
                    if (Character.isLetter(c) || c == '-' || Character.isDigit(c)) {
                        int start = i;
                        while (i < length) {
                            char ch = s.charAt(i);
                            if (Character.isLetterOrDigit(ch) || ch == '-' || ch == '+' || ch == '.') {
                                i++;
                            } else {
                                break;
                            }
                        }
                        String rawVal = s.substring(start, i);
                        tokens.add(correctLiteral(rawVal));
                    } else {
                        i++;
                    }
                    break;
            }
        }
        tokens.add(new Token(TokenKind.EOF, null));
        return tokens;
    }

    private Token correctLiteral(String raw) {
        int len = raw.length();

        if (len <= 5) {
            char first = raw.charAt(0);
            switch (Character.toLowerCase(first)) {
                case 't':
                    if (matchLiteral(raw, "true")) {
                        return Token.TRUE;
                    }
                    break;
                case 'f':
                    if (matchLiteral(raw, "false")) {
                        return Token.FALSE;
                    }
                    break;
                case 'n':
                    if (matchLiteral(raw, "null")) {
                        return Token.NULL;
                    }
                    break;
            }
        }
        Matcher m = NUM_PATTERN.matcher(raw);
        if (m.matches()) {
            return new Token(TokenKind.NUMBER, raw);
        }
        m = ID_PATTERN.matcher(raw);
        if (m.matches()) {
            return new Token(TokenKind.STRING, raw);
        }
        return new Token(TokenKind.UNKNOWN, raw);
    }

    private boolean matchLiteral(String raw, String full) {

        int len = raw.length();
        if (len > full.length()) return false;
        for (int i = 0; i < len; i++) {
            if (Character.toLowerCase(raw.charAt(i)) != full.charAt(i)) {
                return false;
            }
        }
        return true;
    }
}