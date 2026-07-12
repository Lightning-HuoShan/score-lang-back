grammar ScoreLang;

// ============ 词法规则 ============
// 控制指令
At          : '@' ;

KeySet         : 'key' ;
TempoSet       : 'tempo' ;
TimeSet        : 'time' ;
TitleSet       : 'title' ;

MAJOR          : 'major' ;
MINOR          : 'minor' ;

// 结构关键字
Track       : 'track' ;
Section     : 'section' ;
Repeat      : 'repeat' ;
Instrument     : 'inst' ;
InstList       : ( 'piano' ) ;

// 标点
Lbrace      : '{' ;
Rbrace      : '}' ;
Lbrack      : '[' ;
Rbrack      : ']' ;
Lpar        : '(' ;
Rpar        : ')' ;
Bar         : '|' ;
Comma       : ',' ;
Slash       : '/' ;
Dot         : '.' ;
For         : 'f' ;
Rest        : 'R' ;

// 音名（必须大写）
NoteName    : [CDEFGAB] ;

// 变音记号
Accidental  : ('#' | 'b' | 'bb' | 'x' | '=') ;

// 数字
Number      : [0-9]+ ;

ChordQuality: ( 'maj' | 'min' | 'm' | 'dim' | 'aug' | 'sus' ) ;
ChordAlter  : ( 'add' | 'flat' | 'sharp' | 'no' );

// 字符串
StringLit   : '"' ~["]* '"' ;

// 注释和空白
Comment     : '#' ~[\r\n]* -> skip ;
Ws          : [ \t\r\n]+ -> skip ;

// ============ 语法规则 ============
// 总谱
score       : global* track+ EOF ;

// 全局控制
global      : At key_control
            | At tempo_control
            | At time_control
            | At title_control
            ;

key_control     : KeySet Lpar pitchClass (MAJOR | MINOR) Rpar ;
tempo_control   : TempoSet Lpar Number Rpar ;
time_control    : TimeSet Lpar Number Slash Number Rpar ;
title_control   : TitleSet Lpar StringLit Rpar ;

// 乐器声部
track       : Track StringLit ( instrumentAssign )? Lbrace section* Rbrace ;
instrumentAssign : Instrument Lpar InstList Rpar;

// 段落
section      : Section StringLit (repeatAssign)? Lbrace measure* Rbrace ;
repeatAssign : Repeat Lpar Number Rpar ;

// 小节
measure     : event* Bar ;

// 事件
event       : note
            | rest
            | chord
            | local_control
            ;

// 单音符：C4f4 / D#5.2
note        : pitch duration;

// 休止符：Rf4 / R.2
rest        : Rest duration ;

// 和弦：[Cmaj7/G3]f4 / [G7]f4
chord       : Lbrack chord_symbol Slash pitch Rbrack duration
            | Lbrack chord_symbol Rbrack duration
            ;

chord_symbol    : pitchClass chord_quality ;
chord_quality   : ChordQuality Number? (ChordAlter Number)*
                | Number (ChordAlter Number)*
                ;

// 通用模块
pitch           : pitchClass Number? ;
pitchClass      : NoteName Accidental? ;
duration        : ((For | Dot) Number) ;

// 局部控制
local_control   : At key_control
                | At tempo_control
                | At time_control
                ;