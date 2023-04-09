# <a id=""></a> Memorial Implementation Notes

## Table of contents

- [CLI application](#cli) 
	- [Scan](#cli+scan) 
- [Core](#core) 
	- [API](#core+api) 
	- [Collector](#core+collector) 
	- [Model](#core+model) 
		- [Attributes](#core+model+attributes) 
	- [Parser](#core+parser) 
	- [Renderer](#core+renderer) 
		- [Markdown](#core+renderer+markdown) 
		- [Staging](#core+renderer+staging) 

## <a id="cli"></a> CLI application

> The application is primarily designed to be run in non-interactive mode (e.g. as a pre-commit
> hook or during CI). Because of that reason and to emphasize using VCS for anything important,
> practically all parameters are read from a configuration file instead of command-line arguments. 

at [memorial-cli\src\cli\app.rs (line 13)](https://github.com/Kostassoid/memorial/blob/master/memorial-cli/src/cli/app.rs#L13)



> `scan` command is the only one implemented so far but it's not made a default
> because of the likely future extensions. 

at [memorial-cli\src\cli\app.rs (line 21)](https://github.com/Kostassoid/memorial/blob/master/memorial-cli/src/cli/app.rs#L21)



### <a id="cli+scan"></a> Scan

> Even though the overall design and the config model allow for using multiple renderers, this
> feels like a rabbit hole of over-generalization.
> So the idea is to keep Markdown as the one and only renderer until the rest of the project is
> mature enough and there's a clear(er) vision of the roadmap. 

at [memorial-cli\src\cli\scan.rs (line 128)](https://github.com/Kostassoid/memorial/blob/master/memorial-cli/src/cli/scan.rs#L128)



## <a id="core"></a> Core

### <a id="core+api"></a> API

> Event-based callback system allows to decouple core logic from UI without complicating
> abstractions. Also works really well in unit tests.
> 
> Primarily used by [Collector](#core+collector) so far but can be extended easily if needed. 

at [memorial-core\src\api\events.rs (line 5)](https://github.com/Kostassoid/memorial/blob/master/memorial-core/src/api/events.rs#L5)



### <a id="core+collector"></a> Collector

> Ignoring parsing errors on collected quotes on (1,1) position to reduce false warnings. 

at [memorial-core\src\collector\collector.rs (line 70)](https://github.com/Kostassoid/memorial/blob/master/memorial-core/src/collector/collector.rs#L70)




_Mentioned in:_
- [API](#core+api) 
### <a id="core+model"></a> Model

#### <a id="core+model+attributes"></a> Attributes

> Attributes are either attached to the node or control the collection process. 

at [memorial-core\src\model\attributes.rs (line 1)](https://github.com/Kostassoid/memorial/blob/master/memorial-core/src/model/attributes.rs#L1)



> Some attributes are used internally to enrich collected knowledge tree with some valuable context,
> like the timestamp of document generation. These attributes are not supposed to be used by end users
> directly. As a convention, these internal attributes are prefixed with "!", although this is not
> enforced through the parser currently. 

at [memorial-core\src\model\attributes.rs (line 7)](https://github.com/Kostassoid/memorial/blob/master/memorial-core/src/model/attributes.rs#L7)



### <a id="core+parser"></a> Parser

> Handling of the indentations should be ideally done within the generated parser.
> But due to the lack of experience with Pest, this is done as a draft implementation
> using additional post-processing step. This would likely create additional challenge
> in case of multi-line comments using single-line syntax. 

at [memorial-macros\src\lib.rs (line 18)](https://github.com/Kostassoid/memorial/blob/master/memorial-macros/src/lib.rs#L18)



### <a id="core+renderer"></a> Renderer

#### <a id="core+renderer+markdown"></a> Markdown

> One possible future improvement is allowing to render the collected notes into multiple files.
> This can be user-controlled by using attributes. Hence that's how output file path is
> passed to the renderer. But, for now, only a value from the root node is used. 

at [memorial-core\src\renderer\markdown.rs (line 28)](https://github.com/Kostassoid/memorial/blob/master/memorial-core/src/renderer/markdown.rs#L28)



> The renderer is currently implemented using low level string builders. This seemed like a good idea
> during the initial development phase as integrating with template engine libraries would require
> preparing the data in a certain way, which, depending on the engine implementation could limit the
> features exposed from the `Renderer`. 

at [memorial-core\src\renderer\markdown.rs (line 41)](https://github.com/Kostassoid/memorial/blob/master/memorial-core/src/renderer/markdown.rs#L41)



#### <a id="core+renderer+staging"></a> Staging

> `StagingArea` acts as an intermediate temp file system for keeping the rendered files until the rendering
> is complete. The files can then be written down to the final location in one go.
> 
> Writing down the rendered files directly can be problematic. In case of errors these files
> can end up being arbitrarily broken. Having the documentation checked in into VCS can help with
> restoring the original state, but this is an unnecessary limitation. Also it's still an extra step
> which can be avoided. 

at [memorial-core\src\renderer\staging.rs (line 8)](https://github.com/Kostassoid/memorial/blob/master/memorial-core/src/renderer/staging.rs#L8)



> Staging in the current implementation stores data in memory. This approach simplifies design but
> obviously won't scale well. For the future versions, some memory independent storage should be used.
> E.g. `/tmp`. 

at [memorial-core\src\renderer\staging.rs (line 65)](https://github.com/Kostassoid/memorial/blob/master/memorial-core/src/renderer/staging.rs#L65)




---
<sub>Generated by [Memorial](https://github.com/Kostassoid/memorial) v0.1.0 at _2023-04-09 21:47:23_.</sub>