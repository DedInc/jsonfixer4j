package com.github.dedinc.jsonfixer4j;

import java.util.List;
import java.util.Map;

public final class JSONSerializer {

    private static final char[] HEX_DIGITS = "0123456789abcdef".toCharArray();

    public String serialize(Object obj) {

        StringBuilder sb = new StringBuilder(256);
        serialize(obj, sb);
        return sb.toString();
    }

    private void serialize(Object obj, StringBuilder sb) {
        if (obj == null) {
            sb.append("null");
        } else if (obj instanceof Boolean || obj instanceof Number) {
            sb.append(obj.toString());
        } else if (obj instanceof String) {
            appendEscapedString((String) obj, sb);
        } else if (obj instanceof List) {
            List<?> list = (List<?>) obj;
            sb.append('[');
            int size = list.size();
            for (int i = 0; i < size; i++) {
                if (i > 0) {
                    sb.append(", ");
                }
                serialize(list.get(i), sb);
            }
            sb.append(']');
        } else if (obj instanceof Map) {
            @SuppressWarnings("unchecked")
            Map<Object, Object> map = (Map<Object, Object>) obj;
            sb.append('{');
            boolean first = true;
            for (Map.Entry<Object, Object> entry : map.entrySet()) {
                if (!first) {
                    sb.append(", ");
                }
                first = false;
                serialize(entry.getKey(), sb);
                sb.append(": ");
                serialize(entry.getValue(), sb);
            }
            sb.append('}');
        } else {
            appendEscapedString(obj.toString(), sb);
        }
    }

    private void appendEscapedString(String str, StringBuilder sb) {
        sb.append('"');
        final int len = str.length();
        for (int i = 0; i < len; i++) {
            char ch = str.charAt(i);
            switch (ch) {
                case '\\':
                    sb.append("\\\\");
                    break;
                case '"':
                    sb.append("\\\"");
                    break;
                case '\b':
                    sb.append("\\b");
                    break;
                case '\f':
                    sb.append("\\f");
                    break;
                case '\n':
                    sb.append("\\n");
                    break;
                case '\r':
                    sb.append("\\r");
                    break;
                case '\t':
                    sb.append("\\t");
                    break;
                default:
                    if (ch < 0x20) {
                        sb.append("\\u");
                        sb.append(HEX_DIGITS[(ch >> 12) & 0xF]);
                        sb.append(HEX_DIGITS[(ch >> 8) & 0xF]);
                        sb.append(HEX_DIGITS[(ch >> 4) & 0xF]);
                        sb.append(HEX_DIGITS[ch & 0xF]);
                    } else {
                        sb.append(ch);
                    }
                    break;
            }
        }
        sb.append('"');
    }
}