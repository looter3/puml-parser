pub const REGEX_ERROR_MESSAGE: &str = "Failed to compile class regex";
pub const CLASS_REGEX: &str = "class (\\w+)";
pub const INTERFACE_REGEX: &str = "implements\\s+([A-Za-z_][A-Za-z0-9_]*(?:<[^>]*>)?)";
pub const PARENT_CLASS_REGEX: &str = "extends\\s+([A-Za-z_][A-Za-z0-9_]*(?:<[^>]*>)?)";
pub const METHOD_REGEX: &str = r"([+\-#])\s*([\w<>\\[\\]]+)\s*:\s*(\w+)\s*\(([^)]*)\)";
pub const PARAMETER_REGEX: &str = r"(\w+) (\w+)";
pub const FIELD_REGEX: &str = r"[-+#] (\w+) (\w+)";
pub const ANNOTATION_REGEX: &str = r"(?<=<<@)\w+(?=>>)";

