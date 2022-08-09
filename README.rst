
Small and dependency free  package to infer file type and MIME
type checking the `magic numbers`_ signature of a file or buffer.

This is a Rust port from `filetype`_ Go package.

Features
--------

-  Simple and friendly API
-  Supports a `wide range`_ of file types
-  Provides file extension and MIME type inference
-  File discovery by extension or MIME type
-  File discovery by kind (image, video, audio…)
-  `Pluggable`_: add new custom type matchers
-  `Fast`_, even processing large files
-  Only first 261 bytes representing the max file header is required, so
   you can just `pass a list of bytes`_
-  Cross-platform file recognition



Supported types
---------------

Image
^^^^^

-  **dwg** - ``image/vnd.dwg``
-  **xcf** - ``image/x-xcf``
-  **jpg** - ``image/jpeg``
-  **jpx** - ``image/jpx``
-  **png** - ``image/png``
-  **apng** - ``image/apng``
-  **gif** - ``image/gif``
-  **webp** - ``image/webp``
-  **cr2** - ``image/x-canon-cr2``
-  **tif** - ``image/tiff``
-  **bmp** - ``image/bmp``
-  **jxr** - ``image/vnd.ms-photo``
-  **psd** - ``image/vnd.adobe.photoshop``
-  **ico** - ``image/x-icon``
-  **heic** - ``image/heic``

Video
^^^^^

-  **3gp** - ``video/3gpp``
-  **mp4** - ``video/mp4``
-  **m4v** - ``video/x-m4v``
-  **mkv** - ``video/x-matroska``
-  **webm** - ``video/webm``
-  **mov** - ``video/quicktime``
-  **avi** - ``video/x-msvideo``
-  **wmv** - ``video/x-ms-wmv``
-  **mpg** - ``video/mpeg``
-  **flv** - ``video/x-flv``

Audio
^^^^^

-  **aac** - ``audio/aac``
-  **mid** - ``audio/midi``
-  **mp3** - ``audio/mpeg``
-  **m4a** - ``audio/mp4``
-  **ogg** - ``audio/ogg``
-  **flac** - ``audio/x-flac``
-  **wav** - ``audio/x-wav``
-  **amr** - ``audio/amr``
-  **aiff** - ``audio/x-aiff``

Archive
^^^^^^^

-  **br** - ``application/x-brotli``
-  **rpm** - ``application/x-rpm``
-  **dcm** - ``application/dicom``
-  **epub** - ``application/epub+zip``
-  **zip** - ``application/zip``
-  **tar** - ``application/x-tar``
-  **rar** - ``application/x-rar-compressed``
-  **gz** - ``application/gzip``
-  **bz2** - ``application/x-bzip2``
-  **7z** - ``application/x-7z-compressed``
-  **xz** - ``application/x-xz``
-  **pdf** - ``application/pdf``
-  **exe** - ``application/x-msdownload``
-  **swf** - ``application/x-shockwave-flash``
-  **rtf** - ``application/rtf``
-  **eot** - ``application/octet-stream``
-  **ps** - ``application/postscript``
-  **sqlite** - ``application/x-sqlite3``
-  **nes** - ``application/x-nintendo-nes-rom``
-  **crx** - ``application/x-google-chrome-extension``
-  **cab** - ``application/vnd.ms-cab-compressed``
-  **deb** - ``application/x-deb``
-  **ar** - ``application/x-unix-archive``
-  **Z** - ``application/x-compress``
-  **lzo** - ``application/x-lzop``
-  **lz** - ``application/x-lzip``
-  **lz4** - ``application/x-lz4``
-  **zstd** - ``application/zstd``

Font
^^^^

-  **woff** - ``application/font-woff``
-  **woff2** - ``application/font-woff``
-  **ttf** - ``application/font-sfnt``
-  **otf** - ``application/font-sfnt``

Application
^^^^^^^^^^^ 

-  **wasm** - ``application/wasm``

.. _magic numbers: https://en.wikipedia.org/wiki/Magic_number_(programming)#Magic_numbers_in_files
.. _filetype: https://github.com/h2non/filetype
.. _wide range: #supported-types
.. _Pluggable: #add-additional-file-type-matchers
.. _Fast: #benchmarks
.. _pass a list of bytes: #file-header

