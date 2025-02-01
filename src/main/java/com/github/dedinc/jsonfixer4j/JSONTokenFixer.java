package com.github.dedinc.jsonfixer4j;

import java.util.*;

class JSONTokenFixer {

    public static List<Token> fixTokens(List<Token> tokens) {
        List<Token> fixed = new ArrayList<>();
        Deque<TokenKind> stack = new ArrayDeque<>();
        Map<TokenKind, TokenKind> matching = new HashMap<>();
        matching.put(TokenKind.LBRACE, TokenKind.RBRACE);
        matching.put(TokenKind.LBRACKET, TokenKind.RBRACKET);

        for (Token token : tokens) {
            if (token.kind == TokenKind.LBRACE || token.kind == TokenKind.LBRACKET) {
                fixed.add(token);
                stack.push(matching.get(token.kind));
            } else if (token.kind == TokenKind.RBRACE || token.kind == TokenKind.RBRACKET) {
                if (!stack.isEmpty()) {
                    TokenKind expected = stack.peek();
                    if (token.kind == expected) {
                        stack.pop();
                        fixed.add(token);
                    } else {

                        if (expected == TokenKind.RBRACKET) {
                            fixed.add(new Token(TokenKind.RBRACKET, "]"));
                            stack.pop();
                            if (!stack.isEmpty() && token.kind == stack.peek()) {
                                stack.pop();
                                fixed.add(token);
                            } else {
                                fixed.add(token);
                            }
                        } else {
                            fixed.add(token);
                        }
                    }
                } else {
                    fixed.add(token);
                }
            } else {
                fixed.add(token);
            }
        }
        while (!stack.isEmpty()) {
            TokenKind exp = stack.pop();
            if (exp == TokenKind.RBRACE) {
                fixed.add(new Token(TokenKind.RBRACE, "}"));
            } else if (exp == TokenKind.RBRACKET) {
                fixed.add(new Token(TokenKind.RBRACKET, "]"));
            }
        }
        return fixed;
    }
}