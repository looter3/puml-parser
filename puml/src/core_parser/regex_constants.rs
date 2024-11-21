pub const REGEX_ERROR_MESSAGE: &str = "Failed to compile class regex";

// Class
pub const CLASS_PATTERN: &str = "class (\\w+)";
pub const IMPL_INTERFACE_PATTERN: &str = "implements\\s+([A-Za-z_][A-Za-z0-9_]*(?:<[^>]*>)?)";
pub const PARENT_CLASS_PATTERN: &str = "extends\\s+([A-Za-z_][A-Za-z0-9_]*(?:<[^>]*>)?)";
pub const METHOD_PATTERN: &str = r"([+\-#])\s*([\w<>\\[\\]]+)\s*:\s*(\w+)\s*\(([^)]*)\)";
pub const PARAMETER_PATTERN: &str = r"(\w+) (\w+)";
pub const FIELD_PATTERN: &str = r"[-+#] (\w+) (\w+)";
//pub const ANNOTATION_REGEX: &str = r"(?<=<<@)\w+(?=>>)";

// Interface
pub const INTERFACE_PATTERN: &str = "interface (\\w+)";
