package com.github.dedinc.jsonfixer4j;

import java.util.LinkedHashMap;
import java.util.List;

public final class JSONAutoCorrector {

    private final JSONTokenizer tokenizer;
    private final JSONParser parser;
    private final JSONSerializer serializer;

    public JSONAutoCorrector() {
        this.tokenizer = new JSONTokenizer();
        this.parser = new JSONParser();
        this.serializer = new JSONSerializer();
    }

    public String autocorrect(String s) {
        List<Token> tokens = tokenizer.tokenize(s);
        List<Token> fixedTokens = JSONTokenFixer.fixTokens(tokens);
        JSONParser.ParseResult pr = parser.parse(fixedTokens, 0);
        Object result = pr.value;
        if (result == null) {
            result = new LinkedHashMap<>();
        }
        return serializer.serialize(result);
    }
}
