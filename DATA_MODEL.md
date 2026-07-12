# ScoreLang 数据结构层次

---

## Score 层次结构图

```
Score
├── global: GlobalParams
│   ├── key: Key
│   │   ├── tonic: Pitch
│   │   ├── mode: Mode
│   │   ├── sharp_count: u8
│   │   └── flat_count: u8
│   ├── tempo: Tempo (bpm: u32)
│   ├── time_signature: TimeSignature
│   │   ├── beats_per_bar: u8
│   │   └── beat_unit: NoteValue
│   └── title: Option<String>
└── tracks: Vec<Track>
    └── Track
        ├── name: String
        ├── instrument: InstrumentKind
        └── sections: Vec<Section>
            └── Section
                ├── name: String
                ├── Repeat: u32
                ├── local_override: SectionOverride
                │   ├── key: Option<Key>
                │   ├── tempo: Option<Tempo>
                │   └── time_signature: Option<TimeSignature>
                └── measures: Vec<Measure>
                    └── Measure
                        └── events: Vec<Activity>
                            ├── PlayElement(MusicElement)
                            │   ├── Note(Note)
                            │   │   ├── pitch: Pitch
                            │   │   │   ├── letter: LetterName
                            │   │   │   ├── Accidental: Accidental
                            │   │   │   └── Octave: u8
                            │   │   ├── value: NoteValue
                            │   │   └── dotted: bool
                            │   ├── Rest(Rest)
                            │   │   ├── value: NoteValue
                            │   │   └── dotted: bool
                            │   └── Chord(ChordNote)
                            │       ├── chord: Chord
                            │       │   ├── root: Pitch
                            │       │   ├── kind: ChordKind
                            │       │   │   ├── quality: ChordQuality
                            │       │   │   ├── extension: ChordExtension
                            │       │   │   └── alters: Vec<ChordAlter>
                            │       │   ├── tones: Vec<Pitch>
                            │       │   ├── bass: Option<Pitch>
                            │       │   └── inversion: u8
                            │       ├── value: NoteValue
                            │       └── dotted: bool
                            ├── ChangeKey(Key)
                            ├── ChangeTimeSig(TimeSignature)
                            └── ChangeTempo(Tempo)
```

