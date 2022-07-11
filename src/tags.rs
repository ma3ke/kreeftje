use console::style;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Tag {
    // compsci
    /// 'ai' Artificial Intelligence, Machine Learning
    Ai,
    /// 'compsci' Other computer science/programming
    Compsci,
    /// 'distributed' Distributed systems
    Distributed,
    /// 'formalmethods' Formal methods
    Formalmethods,
    /// 'graphics' Graphics programming
    Graphics,
    /// 'networking' Networking
    Networking,
    /// 'osdev' Operating system design and development when no specific OS tag exists
    Osdev,
    /// 'plt' Programming language theory, types, design
    Plt,
    /// 'programming' Use when every tag or no specific tag applies
    Programming,

    // culture
    /// 'culture' Technical communities and culture
    Culture,
    /// 'law' Law, patents, and licensing
    Law,
    /// 'person' Stories about particular persons
    Person,
    /// 'philosophy' Philosophy
    Philosophy,

    // field
    /// 'cogsci' Cognitive Science
    Cogsci,
    /// 'cryptography' Cryptography
    Cryptography,
    /// 'education' Education
    Education,
    /// 'finance' Finance and economics
    Finance,
    /// 'hardware' Hardware
    Hardware,
    /// 'math' Mathematics
    Math,
    /// 'science' It's "Science"
    Science,

    // format
    /// 'ask' Ask Lobsters
    Ask,
    /// 'audio' Link to audio (podcast, interview)
    Audio,
    /// 'book' Link to a book (not an ad or review)
    Book,
    /// 'pdf' Link to a PDF document
    Pdf,
    /// 'show' Show Lobsters / Projects
    Show,
    /// 'slides' Slide deck
    Slides,
    /// 'transcript' Includes transcript of audio or video
    Transcript,
    /// 'video' Link to a video
    Video,

    // genre
    /// 'art' Art
    Art,
    /// 'event' Events, conferences, and meetups
    Event,
    /// 'historical' History and retrospectives (not for things that happen to be old)
    Historical,
    /// 'job' Employment/Internship opportunities
    Job,
    /// 'news' General news and current events | inactive
    News,
    /// 'rant' Rants and raves
    Rant,
    /// 'release' Software releases and announcements
    Release,
    /// 'satire' Satirical writing
    Satire,
    /// 'interaction'
    Interaction,
    /// 'a11y' accessibility, assistive technology, standards
    A11y,
    /// 'design' Visual design
    Design,
    /// 'visualization' Data visualization
    Visualization,

    // languages
    /// 'apl' Array Programming Languages such as APL, J, and K
    Apl,
    /// 'assembly' Assembly programming
    Assembly,
    /// 'c' C programming
    C,
    /// 'c++' C++ programming
    Cpp,
    /// 'clojure' Clojure programming
    Clojure,
    /// 'css' Cascading Style Sheets
    Css,
    /// 'd' D programming
    D,
    /// 'dotnet' C#, F#, .NET programming
    Dotnet,
    /// 'elixir' Elixir programming
    Elixir,
    /// 'elm' Elm programming
    Elm,
    /// 'erlang' Erlang development
    Erlang,
    /// 'fortran' Fortran programming
    Fortran,
    /// 'go' Golang programming
    Go,
    /// 'haskell' Haskell programming
    Haskell,
    /// 'java' Java programming
    Java,
    /// 'javascript' Javascript programming
    Javascript,
    /// 'kotlin' Kotlin programming
    Kotlin,
    /// 'lisp' Lisp and Scheme programming
    Lisp,
    /// 'lua' Lua programming
    Lua,
    /// 'ml' MetaLanguage, OCaml programming
    Ml,
    /// 'nodejs' Node.js programming
    NodeJS,
    /// 'objectivec' Objective-C programming
    ObjectiveC,
    /// 'perl' Perl programming
    Perl,
    /// 'php' PHP programming
    Php,
    /// 'python' Python programming
    Python,
    /// 'ruby' Ruby programming
    Ruby,
    /// 'rust' Rust programming
    Rust,
    /// 'scala' Scala programming
    Scala,
    /// 'swift' Swift programming
    Swift,
    /// 'zig' Zig programming
    Zig,

    // lobsters
    /// 'announce' Site anouncement
    Announce,
    /// 'interview' Lobsters interviews
    Interview,
    /// 'meta' Lobsters-related bikeshedding - report bugs at https://github.com/lobsters/lobsters
    Meta,
    //
    // os
    /// 'android' Android
    Android,
    /// 'dragonflybsd' DragonFly BSD
    DragonflyBSD,
    /// 'freebsd' FreeBSD
    FreeBSD,
    /// 'illumos' illumos
    Illumos,
    /// 'ios' Apple iOS
    Ios,
    /// 'linux' Linux
    Linux,
    /// 'mac' Apple macOS
    Mac,
    /// 'netbsd' NetBSD
    NetBSD,
    /// 'nix' package manager, distribution, and related systems like guix
    Nix,
    /// 'openbsd' OpenBSD
    OpenBSD,
    /// 'unix' *nix
    Unix,
    /// 'windows' Windows
    Windows,

    // platforms
    /// 'browsers' Web browsers
    Browsers,
    /// 'email' e-mail
    Email,
    /// 'games' Game design and study
    Games,
    /// 'ipv6' IPv6 | inactive
    Ipv6,
    /// 'merkle-trees' And related similar data structures. Not business/scam news.
    MerkleTrees,
    /// 'mobile' Mobile app/web development
    Mobile,
    /// 'wasm' webassembly
    Wasm,
    /// 'web' Web development and news
    Web,

    // practices
    /// 'api' API development/implementation
    Api,
    /// 'debugging' Debugging techniques
    Debugging,
    /// 'devops' DevOps
    Devops,
    /// 'performance' Performance and optimization
    Performance,
    /// 'practices' Development and team practices
    Practices,
    /// 'privacy' Privacy
    Privacy,
    /// 'reversing' Reverse engineering
    Reversing,
    /// 'scaling' Scaling and architecture
    Scaling,
    /// 'security' Netsec, appsec, and infosec
    Security,
    /// 'testing' Software testing
    Testing,
    /// 'virtualization' Virtualization
    Virtualization,

    // tools
    /// 'compilers' Compiler design
    Compilers,
    /// 'databases' Databases (SQL, NoSQL)
    Databases,
    /// 'emacs' Emacs editor
    Emacs,
    /// 'systemd' Linux systemd
    Systemd,
    /// 'vcs' Git and other version control systems
    Vcs,
    /// 'vim' Vim editor
    Vim,
}

impl FromStr for Tag {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Tag::*;
        let ret = match s {
            "ai" => Ai,
            "compsci" => Compsci,
            "distributed" => Distributed,
            "formalmethods" => Formalmethods,
            "graphics" => Graphics,
            "networking" => Networking,
            "osdev" => Osdev,
            "plt" => Plt,
            "programming" => Programming,
            "culture" => Culture,
            "law" => Law,
            "person" => Person,
            "philosophy" => Philosophy,
            "cogsci" => Cogsci,
            "cryptography" => Cryptography,
            "education" => Education,
            "finance" => Finance,
            "hardware" => Hardware,
            "math" => Math,
            "science" => Science,
            "ask" => Ask,
            "audio" => Audio,
            "book" => Book,
            "pdf" => Pdf,
            "show" => Show,
            "slides" => Slides,
            "transcript" => Transcript,
            "video" => Video,
            "art" => Art,
            "event" => Event,
            "historical" => Historical,
            "job" => Job,
            "news" => News,
            "rant" => Rant,
            "release" => Release,
            "satire" => Satire,
            "interaction" => Interaction,
            "a11y" => A11y,
            "design" => Design,
            "visualization" => Visualization,
            "apl" => Apl,
            "assembly" => Assembly,
            "c" => C,
            "c++" => Cpp,
            "clojure" => Clojure,
            "css" => Css,
            "d" => D,
            "dotnet" => Dotnet,
            "elixir" => Elixir,
            "elm" => Elm,
            "erlang" => Erlang,
            "fortran" => Fortran,
            "go" => Go,
            "haskell" => Haskell,
            "java" => Java,
            "javascript" => Javascript,
            "kotlin" => Kotlin,
            "lisp" => Lisp,
            "lua" => Lua,
            "ml" => Ml,
            "nodejs" => NodeJS,
            "objectivec" => ObjectiveC,
            "perl" => Perl,
            "php" => Php,
            "python" => Python,
            "ruby" => Ruby,
            "rust" => Rust,
            "scala" => Scala,
            "swift" => Swift,
            "zig" => Zig,
            "announce" => Announce,
            "interview" => Interview,
            "meta" => Meta,
            "android" => Android,
            "dragonflybsd" => DragonflyBSD,
            "freebsd" => FreeBSD,
            "illumos" => Illumos,
            "ios" => Ios,
            "linux" => Linux,
            "mac" => Mac,
            "netbsd" => NetBSD,
            "nix" => Nix,
            "openbsd" => OpenBSD,
            "unix" => Unix,
            "windows" => Windows,
            "browsers" => Browsers,
            "email" => Email,
            "games" => Games,
            "ipv6" => Ipv6,
            "merkle-trees" => MerkleTrees,
            "mobile" => Mobile,
            "wasm" => Wasm,
            "web" => Web,
            "api" => Api,
            "debugging" => Debugging,
            "devops" => Devops,
            "performance" => Performance,
            "practices" => Practices,
            "privacy" => Privacy,
            "reversing" => Reversing,
            "scaling" => Scaling,
            "security" => Security,
            "testing" => Testing,
            "virtualization" => Virtualization,
            "compilers" => Compilers,
            "databases" => Databases,
            "emacs" => Emacs,
            "systemd" => Systemd,
            "vcs" => Vcs,
            "vim" => Vim,
            t => return Err(format!("No tag with the name '{t}' exists")),
        };
        Ok(ret)
    }
}

impl ToString for Tag {
    fn to_string(&self) -> String {
        use Tag::*;
        let text = match self {
            Ai => "ai",
            Compsci => "compsci",
            Distributed => "distributed",
            Formalmethods => "formalmethods",
            Graphics => "graphics",
            Networking => "networking",
            Osdev => "osdev",
            Plt => "plt",
            Programming => "programming",
            Culture => "culture",
            Law => "law",
            Person => "person",
            Philosophy => "philosophy",
            Cogsci => "cogsci",
            Cryptography => "cryptography",
            Education => "education",
            Finance => "finance",
            Hardware => "hardware",
            Math => "math",
            Science => "science",
            Ask => "ask",
            Audio => "audio",
            Book => "book",
            Pdf => "pdf",
            Show => "show",
            Slides => "slides",
            Transcript => "transcript",
            Video => "video",
            Art => "art",
            Event => "event",
            Historical => "historical",
            Job => "job",
            News => "news",
            Rant => "rant",
            Release => "release",
            Satire => "satire",
            Interaction => "interaction",
            A11y => "a11y",
            Design => "design",
            Visualization => "visualization",
            Apl => "apl",
            Assembly => "assembly",
            C => "c",
            Cpp => "c++",
            Clojure => "clojure",
            Css => "css",
            D => "d",
            Dotnet => "dotnet",
            Elixir => "elixir",
            Elm => "elm",
            Erlang => "erlang",
            Fortran => "fortran",
            Go => "go",
            Haskell => "haskell",
            Java => "java",
            Javascript => "javascript",
            Kotlin => "kotlin",
            Lisp => "lisp",
            Lua => "lua",
            Ml => "ml",
            NodeJS => "nodejs",
            ObjectiveC => "objectivec",
            Perl => "perl",
            Php => "php",
            Python => "python",
            Ruby => "ruby",
            Rust => "rust",
            Scala => "scala",
            Swift => "swift",
            Zig => "zig",
            Announce => "announce",
            Interview => "interview",
            Meta => "meta",
            Android => "android",
            DragonflyBSD => "dragonflybsd",
            FreeBSD => "freebsd",
            Illumos => "illumos",
            Ios => "ios",
            Linux => "linux",
            Mac => "mac",
            NetBSD => "netbsd",
            Nix => "nix",
            OpenBSD => "openbsd",
            Unix => "unix",
            Windows => "windows",
            Browsers => "browsers",
            Email => "email",
            Games => "games",
            Ipv6 => "ipv6",
            MerkleTrees => "merkletrees",
            Mobile => "mobile",
            Wasm => "wasm",
            Web => "web",
            Api => "api",
            Debugging => "debugging",
            Devops => "devops",
            Performance => "performance",
            Practices => "practices",
            Privacy => "privacy",
            Reversing => "reversing",
            Scaling => "scaling",
            Security => "security",
            Testing => "testing",
            Virtualization => "virtualization",
            Compilers => "compilers",
            Databases => "databases",
            Emacs => "emacs",
            Systemd => "systemd",
            Vcs => "vcs",
            Vim => "vim",
        };
        let code = match self.color() {
            Color::Blue => 117,
            Color::Red => 210,
            Color::Magenta => 102,
            Color::Yellow => 222,
        };
        style(text).color256(code).to_string()
    }
}

enum Color {
    /// Media
    Blue,
    /// Lobsters-related
    Red,
    /// Meta
    Magenta,
    /// All else
    Yellow,
}

impl Tag {
    fn color(&self) -> Color {
        use Tag::*;
        match self {
            Audio | Book | Pdf | Slides | Transcript | Video => Color::Blue,
            Ask | Show | Announce | Interview => Color::Red,
            Meta => Color::Magenta,
            _ => Color::Yellow,
        }
    }
}
