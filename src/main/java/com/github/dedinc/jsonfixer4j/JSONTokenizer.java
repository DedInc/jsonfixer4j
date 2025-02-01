package com.github.dedinc.jsonfixer4j;

import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

class JSONTokenizer {

    public List<Token> tokenize(String s) {
        List<Token> tokens = new ArrayList<>();
        int i = 0;
        int length = s.length();
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
                    continue;
                case '}':
                    tokens.add(new Token(TokenKind.RBRACE, "}"));
                    i++;
                    continue;
                case '[':
                    tokens.add(new Token(TokenKind.LBRACKET, "["));
                    i++;
                    continue;
                case ']':
                    tokens.add(new Token(TokenKind.RBRACKET, "]"));
                    i++;
                    continue;
                case ':':
                    tokens.add(new Token(TokenKind.COLON, ":"));
                    i++;
                    continue;
                case ',':
                    tokens.add(new Token(TokenKind.COMMA, ","));
                    i++;
                    continue;
                case '"':
                    i++;
                    StringBuilder sb = new StringBuilder();
                    while (i < length) {
                        char current = s.charAt(i);
                        if (current == '\\') {
                            if (i + 1 < length) {
                                sb.append(current);
                                sb.append(s.charAt(i + 1));
                                i += 2;
                            } else {
                                i++;
                                break;
                            }
                        } else if (current == '"') {
                            break;
                        } else {
                            sb.append(current);
                            i++;
                        }
                    }
                    if (i < length && s.charAt(i) == '"') {
                        i++;
                    }
                    String strVal = sb.toString();
                    tokens.add(new Token(TokenKind.STRING, strVal));
                    continue;
                default:

                    if (Character.isLetter(c) || c == '-' || Character.isDigit(c)) {
                        int start = i;
                        while (i < length && (Character.isLetterOrDigit(s.charAt(i))
                                || s.charAt(i) == '-' || s.charAt(i) == '+' || s.charAt(i) == '.')) {
                            i++;
                        }
                        String rawVal = s.substring(start, i);
                        Token token = _correctLiteral(rawVal);
                        tokens.add(token);
                        continue;
                    } else {

                        i++;
                        continue;
                    }
            }
        }
        tokens.add(new Token(TokenKind.EOF, null));
        return tokens;
    }

    private Token _correctLiteral(String raw) {
        String lower = raw.toLowerCase();
        if (lower.equals("t") || lower.equals("tr") || lower.equals("tru") || lower.equals("true")) {
            return new Token(TokenKind.TRUE, "true");
        }
        if (lower.equals("f") || lower.equals("fa") || lower.equals("fal") || lower.equals("fals") || lower.equals("false")) {
            return new Token(TokenKind.FALSE, "false");
        }
        if (lower.equals("n") || lower.equals("nu") || lower.equals("nul") || lower.equals("null")) {
            return new Token(TokenKind.NULL, "null");
        }
        Pattern numPattern = Pattern.compile("^-?\\d+(\\.\\d+)?$");
        Matcher m = numPattern.matcher(raw);
        if (m.matches()) {
            return new Token(TokenKind.NUMBER, raw);
        }
        Pattern idPattern = Pattern.compile("^[A-Za-z_]\\w*$");
        m = idPattern.matcher(raw);
        if (m.matches()) {
            return new Token(TokenKind.STRING, raw);
        }
        return new Token(TokenKind.UNKNOWN, raw);
    }
}