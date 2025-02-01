package com.github.dedinc.jsonfixer4j;

import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

class JSONParser {

    static class ParseResult {
        Object value;
        int index;

        public ParseResult(Object value, int index) {
            this.value = value;
            this.index = index;
        }
    }

    public ParseResult parse(List<Token> tokenList, int idx) {
        if (idx >= tokenList.size()) {
            return new ParseResult(null, idx);
        }
        Token token = tokenList.get(idx);
        switch (token.kind) {
            case LBRACE:
                return _parseObject(tokenList, idx + 1);
            case LBRACKET:
                return _parseArray(tokenList, idx + 1);
            case STRING:
                return new ParseResult(token.value, idx + 1);
            case NUMBER:
                try {
                    if (token.value.contains(".")) {
                        return new ParseResult(Double.parseDouble(token.value), idx + 1);
                    } else {
                        return new ParseResult(Integer.parseInt(token.value), idx + 1);
                    }
                } catch (NumberFormatException e) {
                    return new ParseResult(token.value, idx + 1);
                }
            case TRUE:
                return new ParseResult(true, idx + 1);
            case FALSE:
                return new ParseResult(false, idx + 1);
            case NULL:
                return new ParseResult(null, idx + 1);
            case RBRACE:
            case RBRACKET:
            case EOF:
                return new ParseResult(null, idx);
            default:
                return new ParseResult(null, idx + 1);
        }
    }

    private ParseResult _parseObject(List<Token> tokenList, int idx) {
        Map<String, Object> result = new LinkedHashMap<>();
        boolean expectComma = false;
        while (true) {
            if (idx >= tokenList.size()) {
                return new ParseResult(result, idx);
            }
            Token token = tokenList.get(idx);
            if (token.kind == TokenKind.RBRACE) {
                return new ParseResult(result, idx + 1);
            }
            if (token.kind == TokenKind.EOF) {
                return new ParseResult(result, idx);
            }
            if (expectComma) {
                if (token.kind == TokenKind.COMMA) {
                    idx++;
                    expectComma = false;
                    continue;
                } else if (token.kind == TokenKind.RBRACE) {
                    return new ParseResult(result, idx + 1);
                }
                expectComma = false;
            }
            if (token.kind == TokenKind.STRING) {
                String key = token.value;
                idx++;
                if (idx < tokenList.size() && tokenList.get(idx).kind == TokenKind.COLON) {
                    idx++;
                    ParseResult pr = parse(tokenList, idx);
                    result.put(key, pr.value);
                    idx = pr.index;
                    expectComma = true;
                } else {

                    idx++;
                    continue;
                }
            } else {
                idx++;
            }
        }
    }

    private ParseResult _parseArray(List<Token> tokenList, int idx) {
        List<Object> result = new ArrayList<>();
        boolean expectComma = false;
        while (true) {
            if (idx >= tokenList.size()) {
                return new ParseResult(result, idx);
            }
            Token token = tokenList.get(idx);
            if (token.kind == TokenKind.RBRACKET) {
                return new ParseResult(result, idx + 1);
            }
            if (token.kind == TokenKind.EOF) {
                return new ParseResult(result, idx);
            }
            if (expectComma) {
                if (token.kind == TokenKind.COMMA) {
                    idx++;
                    expectComma = false;
                    continue;
                } else if (token.kind == TokenKind.RBRACKET) {
                    return new ParseResult(result, idx + 1);
                } else {
                    idx++;
                    continue;
                }
            }

            if (!(token.kind == TokenKind.LBRACE || token.kind == TokenKind.LBRACKET ||
                    token.kind == TokenKind.STRING || token.kind == TokenKind.NUMBER ||
                    token.kind == TokenKind.TRUE || token.kind == TokenKind.FALSE ||
                    token.kind == TokenKind.NULL)) {
                return new ParseResult(result, idx);
            }
            ParseResult pr = parse(tokenList, idx);
            result.add(pr.value);
            idx = pr.index;
            expectComma = true;
        }
    }
}