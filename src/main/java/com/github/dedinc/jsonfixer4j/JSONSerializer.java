package com.github.dedinc.jsonfixer4j;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

class JSONSerializer {

    public String serialize(Object obj) {
        if (Boolean.TRUE.equals(obj)) {
            return "true";
        } else if (Boolean.FALSE.equals(obj)) {
            return "false";
        } else if (obj == null) {
            return "null";
        } else if (obj instanceof Number) {
            return obj.toString();
        } else if (obj instanceof String) {
            String safeStr = ((String) obj).replace("\\", "\\\\").replace("\"", "\\\"");
            return "\"" + safeStr + "\"";
        } else if (obj instanceof List) {
            @SuppressWarnings("unchecked")
            List<Object> list = (List<Object>) obj;
            List<String> parts = new ArrayList<>();
            for (Object o : list) {
                parts.add(serialize(o));
            }
            return "[" + String.join(", ", parts) + "]";
        } else if (obj instanceof Map) {
            @SuppressWarnings("unchecked")
            Map<Object, Object> map = (Map<Object, Object>) obj;
            List<String> items = new ArrayList<>();
            for (Map.Entry<Object, Object> entry : map.entrySet()) {
                String keyStr = serialize(entry.getKey());
                String valueStr = serialize(entry.getValue());
                items.add(keyStr + ": " + valueStr);
            }
            return "{" + String.join(", ", items) + "}";
        } else {
            return "\"" + obj + "\"";
        }
    }
}