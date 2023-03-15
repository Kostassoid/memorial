/*@[Core/Model/Attributes]
Attributes are either attached to the node or control the collection process.
*/
pub const TITLE: &'static str = "title";
pub const DO_NOT_COLLECT: &'static str = "do-not-collect";

/*@[Core/Model/Attributes]
Some attributes are used internally to enrich collected knowledge tree with some valuable context,
like the timestamp of document generation. These attributes are not supposed to be used by end users
directly. As a convention, these internal attributes are prefixed with "!", although this is not
enforced through the parser currently.
*/
pub const APP_VERSION: &'static str = "!app-version";
pub const TIMESTAMP: &'static str = "!timestamp";
