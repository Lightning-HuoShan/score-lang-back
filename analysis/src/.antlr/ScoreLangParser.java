// Generated from d:/Program/Projects/ScoreAnalysis/analysis/src/ScoreLang.gram by ANTLR 4.13.1
import org.antlr.v4.runtime.atn.*;
import org.antlr.v4.runtime.dfa.DFA;
import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.misc.*;
import org.antlr.v4.runtime.tree.*;
import java.util.List;
import java.util.Iterator;
import java.util.ArrayList;

@SuppressWarnings({"all", "warnings", "unchecked", "unused", "cast", "CheckReturnValue"})
public class ScoreLangParser extends Parser {
	static { RuntimeMetaData.checkVersion("4.13.1", RuntimeMetaData.VERSION); }

	protected static final DFA[] _decisionToDFA;
	protected static final PredictionContextCache _sharedContextCache =
		new PredictionContextCache();
	public static final int
		Track=1, At=2, KeySet=3, Lpar=4, MAJOR=5, MINOR=6, Rpar=7, TempoSet=8, 
		Number=9, TimeSet=10, Slash=11, TitleSet=12, StringLit=13, Lbrace=14, 
		Section=15, Rbrace=16, Instrument=17, InstList=18, Repeat=19, Bar=20, 
		Rest=21, Lbrack=22, Rbrack=23, ChordQuality=24, ChordAlter=25, Comma=26, 
		Octave=27, NoteName=28, Accidental=29, For=30, Dot=31, NoteValue=32;
	public static final int
		RULE_score = 0, RULE_global = 1, RULE_key_control = 2, RULE_tempo_control = 3, 
		RULE_time_control = 4, RULE_title_control = 5, RULE_track = 6, RULE_instrumentAssign = 7, 
		RULE_section = 8, RULE_repeatAssign = 9, RULE_measure = 10, RULE_event = 11, 
		RULE_note = 12, RULE_rest = 13, RULE_chord = 14, RULE_chord_symbol = 15, 
		RULE_chord_quality = 16, RULE_pitch_list = 17, RULE_pitch = 18, RULE_pitchClass = 19, 
		RULE_duration = 20, RULE_local_control = 21;
	private static String[] makeRuleNames() {
		return new String[] {
			"score", "global", "key_control", "tempo_control", "time_control", "title_control", 
			"track", "instrumentAssign", "section", "repeatAssign", "measure", "event", 
			"note", "rest", "chord", "chord_symbol", "chord_quality", "pitch_list", 
			"pitch", "pitchClass", "duration", "local_control"
		};
	}
	public static final String[] ruleNames = makeRuleNames();

	private static String[] makeLiteralNames() {
		return new String[] {
		};
	}
	private static final String[] _LITERAL_NAMES = makeLiteralNames();
	private static String[] makeSymbolicNames() {
		return new String[] {
			null, "Track", "At", "KeySet", "Lpar", "MAJOR", "MINOR", "Rpar", "TempoSet", 
			"Number", "TimeSet", "Slash", "TitleSet", "StringLit", "Lbrace", "Section", 
			"Rbrace", "Instrument", "InstList", "Repeat", "Bar", "Rest", "Lbrack", 
			"Rbrack", "ChordQuality", "ChordAlter", "Comma", "Octave", "NoteName", 
			"Accidental", "For", "Dot", "NoteValue"
		};
	}
	private static final String[] _SYMBOLIC_NAMES = makeSymbolicNames();
	public static final Vocabulary VOCABULARY = new VocabularyImpl(_LITERAL_NAMES, _SYMBOLIC_NAMES);

	/**
	 * @deprecated Use {@link #VOCABULARY} instead.
	 */
	@Deprecated
	public static final String[] tokenNames;
	static {
		tokenNames = new String[_SYMBOLIC_NAMES.length];
		for (int i = 0; i < tokenNames.length; i++) {
			tokenNames[i] = VOCABULARY.getLiteralName(i);
			if (tokenNames[i] == null) {
				tokenNames[i] = VOCABULARY.getSymbolicName(i);
			}

			if (tokenNames[i] == null) {
				tokenNames[i] = "<INVALID>";
			}
		}
	}

	@Override
	@Deprecated
	public String[] getTokenNames() {
		return tokenNames;
	}

	@Override

	public Vocabulary getVocabulary() {
		return VOCABULARY;
	}

	@Override
	public String getGrammarFileName() { return "ScoreLang.gram"; }

	@Override
	public String[] getRuleNames() { return ruleNames; }

	@Override
	public String getSerializedATN() { return _serializedATN; }

	@Override
	public ATN getATN() { return _ATN; }

	public ScoreLangParser(TokenStream input) {
		super(input);
		_interp = new ParserATNSimulator(this,_ATN,_decisionToDFA,_sharedContextCache);
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ScoreContext extends ParserRuleContext {
		public TerminalNode EOF() { return getToken(ScoreLangParser.EOF, 0); }
		public List<GlobalContext> global() {
			return getRuleContexts(GlobalContext.class);
		}
		public GlobalContext global(int i) {
			return getRuleContext(GlobalContext.class,i);
		}
		public List<TerminalNode> Track() { return getTokens(ScoreLangParser.Track); }
		public TerminalNode Track(int i) {
			return getToken(ScoreLangParser.Track, i);
		}
		public ScoreContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_score; }
	}

	public final ScoreContext score() throws RecognitionException {
		ScoreContext _localctx = new ScoreContext(_ctx, getState());
		enterRule(_localctx, 0, RULE_score);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(47);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==At) {
				{
				{
				setState(44);
				global();
				}
				}
				setState(49);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(51); 
			_errHandler.sync(this);
			_la = _input.LA(1);
			do {
				{
				{
				setState(50);
				match(Track);
				}
				}
				setState(53); 
				_errHandler.sync(this);
				_la = _input.LA(1);
			} while ( _la==Track );
			setState(55);
			match(EOF);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class GlobalContext extends ParserRuleContext {
		public TerminalNode At() { return getToken(ScoreLangParser.At, 0); }
		public Key_controlContext key_control() {
			return getRuleContext(Key_controlContext.class,0);
		}
		public Tempo_controlContext tempo_control() {
			return getRuleContext(Tempo_controlContext.class,0);
		}
		public Time_controlContext time_control() {
			return getRuleContext(Time_controlContext.class,0);
		}
		public Title_controlContext title_control() {
			return getRuleContext(Title_controlContext.class,0);
		}
		public GlobalContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_global; }
	}

	public final GlobalContext global() throws RecognitionException {
		GlobalContext _localctx = new GlobalContext(_ctx, getState());
		enterRule(_localctx, 2, RULE_global);
		try {
			setState(65);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,2,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(57);
				match(At);
				setState(58);
				key_control();
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(59);
				match(At);
				setState(60);
				tempo_control();
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(61);
				match(At);
				setState(62);
				time_control();
				}
				break;
			case 4:
				enterOuterAlt(_localctx, 4);
				{
				setState(63);
				match(At);
				setState(64);
				title_control();
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Key_controlContext extends ParserRuleContext {
		public TerminalNode KeySet() { return getToken(ScoreLangParser.KeySet, 0); }
		public TerminalNode Lpar() { return getToken(ScoreLangParser.Lpar, 0); }
		public PitchClassContext pitchClass() {
			return getRuleContext(PitchClassContext.class,0);
		}
		public TerminalNode Rpar() { return getToken(ScoreLangParser.Rpar, 0); }
		public TerminalNode MAJOR() { return getToken(ScoreLangParser.MAJOR, 0); }
		public TerminalNode MINOR() { return getToken(ScoreLangParser.MINOR, 0); }
		public Key_controlContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_key_control; }
	}

	public final Key_controlContext key_control() throws RecognitionException {
		Key_controlContext _localctx = new Key_controlContext(_ctx, getState());
		enterRule(_localctx, 4, RULE_key_control);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(67);
			match(KeySet);
			setState(68);
			match(Lpar);
			setState(69);
			pitchClass();
			setState(70);
			_la = _input.LA(1);
			if ( !(_la==MAJOR || _la==MINOR) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			setState(71);
			match(Rpar);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Tempo_controlContext extends ParserRuleContext {
		public TerminalNode TempoSet() { return getToken(ScoreLangParser.TempoSet, 0); }
		public TerminalNode Lpar() { return getToken(ScoreLangParser.Lpar, 0); }
		public TerminalNode Number() { return getToken(ScoreLangParser.Number, 0); }
		public TerminalNode Rpar() { return getToken(ScoreLangParser.Rpar, 0); }
		public Tempo_controlContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_tempo_control; }
	}

	public final Tempo_controlContext tempo_control() throws RecognitionException {
		Tempo_controlContext _localctx = new Tempo_controlContext(_ctx, getState());
		enterRule(_localctx, 6, RULE_tempo_control);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(73);
			match(TempoSet);
			setState(74);
			match(Lpar);
			setState(75);
			match(Number);
			setState(76);
			match(Rpar);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Time_controlContext extends ParserRuleContext {
		public TerminalNode TimeSet() { return getToken(ScoreLangParser.TimeSet, 0); }
		public TerminalNode Lpar() { return getToken(ScoreLangParser.Lpar, 0); }
		public List<TerminalNode> Number() { return getTokens(ScoreLangParser.Number); }
		public TerminalNode Number(int i) {
			return getToken(ScoreLangParser.Number, i);
		}
		public TerminalNode Slash() { return getToken(ScoreLangParser.Slash, 0); }
		public TerminalNode Rpar() { return getToken(ScoreLangParser.Rpar, 0); }
		public Time_controlContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_time_control; }
	}

	public final Time_controlContext time_control() throws RecognitionException {
		Time_controlContext _localctx = new Time_controlContext(_ctx, getState());
		enterRule(_localctx, 8, RULE_time_control);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(78);
			match(TimeSet);
			setState(79);
			match(Lpar);
			setState(80);
			match(Number);
			setState(81);
			match(Slash);
			setState(82);
			match(Number);
			setState(83);
			match(Rpar);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Title_controlContext extends ParserRuleContext {
		public TerminalNode TitleSet() { return getToken(ScoreLangParser.TitleSet, 0); }
		public TerminalNode Lpar() { return getToken(ScoreLangParser.Lpar, 0); }
		public TerminalNode StringLit() { return getToken(ScoreLangParser.StringLit, 0); }
		public TerminalNode Rpar() { return getToken(ScoreLangParser.Rpar, 0); }
		public Title_controlContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_title_control; }
	}

	public final Title_controlContext title_control() throws RecognitionException {
		Title_controlContext _localctx = new Title_controlContext(_ctx, getState());
		enterRule(_localctx, 10, RULE_title_control);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(85);
			match(TitleSet);
			setState(86);
			match(Lpar);
			setState(87);
			match(StringLit);
			setState(88);
			match(Rpar);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class TrackContext extends ParserRuleContext {
		public TerminalNode Track() { return getToken(ScoreLangParser.Track, 0); }
		public TerminalNode StringLit() { return getToken(ScoreLangParser.StringLit, 0); }
		public TerminalNode Lbrace() { return getToken(ScoreLangParser.Lbrace, 0); }
		public TerminalNode Rbrace() { return getToken(ScoreLangParser.Rbrace, 0); }
		public InstrumentAssignContext instrumentAssign() {
			return getRuleContext(InstrumentAssignContext.class,0);
		}
		public List<TerminalNode> Section() { return getTokens(ScoreLangParser.Section); }
		public TerminalNode Section(int i) {
			return getToken(ScoreLangParser.Section, i);
		}
		public TrackContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_track; }
	}

	public final TrackContext track() throws RecognitionException {
		TrackContext _localctx = new TrackContext(_ctx, getState());
		enterRule(_localctx, 12, RULE_track);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(90);
			match(Track);
			setState(91);
			match(StringLit);
			setState(93);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==Instrument) {
				{
				setState(92);
				instrumentAssign();
				}
			}

			setState(95);
			match(Lbrace);
			setState(99);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==Section) {
				{
				{
				setState(96);
				match(Section);
				}
				}
				setState(101);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(102);
			match(Rbrace);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class InstrumentAssignContext extends ParserRuleContext {
		public TerminalNode Instrument() { return getToken(ScoreLangParser.Instrument, 0); }
		public TerminalNode Lpar() { return getToken(ScoreLangParser.Lpar, 0); }
		public TerminalNode InstList() { return getToken(ScoreLangParser.InstList, 0); }
		public TerminalNode Rpar() { return getToken(ScoreLangParser.Rpar, 0); }
		public InstrumentAssignContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_instrumentAssign; }
	}

	public final InstrumentAssignContext instrumentAssign() throws RecognitionException {
		InstrumentAssignContext _localctx = new InstrumentAssignContext(_ctx, getState());
		enterRule(_localctx, 14, RULE_instrumentAssign);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(104);
			match(Instrument);
			setState(105);
			match(Lpar);
			setState(106);
			match(InstList);
			setState(107);
			match(Rpar);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class SectionContext extends ParserRuleContext {
		public TerminalNode Section() { return getToken(ScoreLangParser.Section, 0); }
		public TerminalNode StringLit() { return getToken(ScoreLangParser.StringLit, 0); }
		public TerminalNode Lbrace() { return getToken(ScoreLangParser.Lbrace, 0); }
		public TerminalNode Rbrace() { return getToken(ScoreLangParser.Rbrace, 0); }
		public RepeatAssignContext repeatAssign() {
			return getRuleContext(RepeatAssignContext.class,0);
		}
		public List<MeasureContext> measure() {
			return getRuleContexts(MeasureContext.class);
		}
		public MeasureContext measure(int i) {
			return getRuleContext(MeasureContext.class,i);
		}
		public SectionContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_section; }
	}

	public final SectionContext section() throws RecognitionException {
		SectionContext _localctx = new SectionContext(_ctx, getState());
		enterRule(_localctx, 16, RULE_section);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(109);
			match(Section);
			setState(110);
			match(StringLit);
			setState(112);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==Repeat) {
				{
				setState(111);
				repeatAssign();
				}
			}

			setState(114);
			match(Lbrace);
			setState(118);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while ((((_la) & ~0x3f) == 0 && ((1L << _la) & 275775492L) != 0)) {
				{
				{
				setState(115);
				measure();
				}
				}
				setState(120);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(121);
			match(Rbrace);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class RepeatAssignContext extends ParserRuleContext {
		public TerminalNode Repeat() { return getToken(ScoreLangParser.Repeat, 0); }
		public TerminalNode Lpar() { return getToken(ScoreLangParser.Lpar, 0); }
		public TerminalNode Number() { return getToken(ScoreLangParser.Number, 0); }
		public TerminalNode Rpar() { return getToken(ScoreLangParser.Rpar, 0); }
		public RepeatAssignContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_repeatAssign; }
	}

	public final RepeatAssignContext repeatAssign() throws RecognitionException {
		RepeatAssignContext _localctx = new RepeatAssignContext(_ctx, getState());
		enterRule(_localctx, 18, RULE_repeatAssign);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(123);
			match(Repeat);
			setState(124);
			match(Lpar);
			setState(125);
			match(Number);
			setState(126);
			match(Rpar);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MeasureContext extends ParserRuleContext {
		public TerminalNode Bar() { return getToken(ScoreLangParser.Bar, 0); }
		public List<EventContext> event() {
			return getRuleContexts(EventContext.class);
		}
		public EventContext event(int i) {
			return getRuleContext(EventContext.class,i);
		}
		public MeasureContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_measure; }
	}

	public final MeasureContext measure() throws RecognitionException {
		MeasureContext _localctx = new MeasureContext(_ctx, getState());
		enterRule(_localctx, 20, RULE_measure);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(131);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while ((((_la) & ~0x3f) == 0 && ((1L << _la) & 274726916L) != 0)) {
				{
				{
				setState(128);
				event();
				}
				}
				setState(133);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(134);
			match(Bar);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class EventContext extends ParserRuleContext {
		public NoteContext note() {
			return getRuleContext(NoteContext.class,0);
		}
		public RestContext rest() {
			return getRuleContext(RestContext.class,0);
		}
		public ChordContext chord() {
			return getRuleContext(ChordContext.class,0);
		}
		public Local_controlContext local_control() {
			return getRuleContext(Local_controlContext.class,0);
		}
		public EventContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_event; }
	}

	public final EventContext event() throws RecognitionException {
		EventContext _localctx = new EventContext(_ctx, getState());
		enterRule(_localctx, 22, RULE_event);
		try {
			setState(140);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case NoteName:
				enterOuterAlt(_localctx, 1);
				{
				setState(136);
				note();
				}
				break;
			case Rest:
				enterOuterAlt(_localctx, 2);
				{
				setState(137);
				rest();
				}
				break;
			case Lbrack:
				enterOuterAlt(_localctx, 3);
				{
				setState(138);
				chord();
				}
				break;
			case At:
				enterOuterAlt(_localctx, 4);
				{
				setState(139);
				local_control();
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class NoteContext extends ParserRuleContext {
		public PitchContext pitch() {
			return getRuleContext(PitchContext.class,0);
		}
		public DurationContext duration() {
			return getRuleContext(DurationContext.class,0);
		}
		public NoteContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_note; }
	}

	public final NoteContext note() throws RecognitionException {
		NoteContext _localctx = new NoteContext(_ctx, getState());
		enterRule(_localctx, 24, RULE_note);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(142);
			pitch();
			setState(143);
			duration();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class RestContext extends ParserRuleContext {
		public TerminalNode Rest() { return getToken(ScoreLangParser.Rest, 0); }
		public DurationContext duration() {
			return getRuleContext(DurationContext.class,0);
		}
		public RestContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_rest; }
	}

	public final RestContext rest() throws RecognitionException {
		RestContext _localctx = new RestContext(_ctx, getState());
		enterRule(_localctx, 26, RULE_rest);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(145);
			match(Rest);
			setState(146);
			duration();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ChordContext extends ParserRuleContext {
		public TerminalNode Lbrack() { return getToken(ScoreLangParser.Lbrack, 0); }
		public Chord_symbolContext chord_symbol() {
			return getRuleContext(Chord_symbolContext.class,0);
		}
		public TerminalNode Slash() { return getToken(ScoreLangParser.Slash, 0); }
		public PitchContext pitch() {
			return getRuleContext(PitchContext.class,0);
		}
		public TerminalNode Rbrack() { return getToken(ScoreLangParser.Rbrack, 0); }
		public DurationContext duration() {
			return getRuleContext(DurationContext.class,0);
		}
		public Pitch_listContext pitch_list() {
			return getRuleContext(Pitch_listContext.class,0);
		}
		public ChordContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_chord; }
	}

	public final ChordContext chord() throws RecognitionException {
		ChordContext _localctx = new ChordContext(_ctx, getState());
		enterRule(_localctx, 28, RULE_chord);
		try {
			setState(172);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,9,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(148);
				match(Lbrack);
				setState(149);
				chord_symbol();
				setState(150);
				match(Slash);
				setState(151);
				pitch();
				setState(152);
				match(Rbrack);
				setState(153);
				duration();
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(155);
				match(Lbrack);
				setState(156);
				chord_symbol();
				setState(157);
				match(Rbrack);
				setState(158);
				duration();
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(160);
				match(Lbrack);
				setState(161);
				pitch_list();
				setState(162);
				match(Slash);
				setState(163);
				pitch();
				setState(164);
				match(Rbrack);
				setState(165);
				duration();
				}
				break;
			case 4:
				enterOuterAlt(_localctx, 4);
				{
				setState(167);
				match(Lbrack);
				setState(168);
				pitch_list();
				setState(169);
				match(Rbrack);
				setState(170);
				duration();
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Chord_symbolContext extends ParserRuleContext {
		public PitchClassContext pitchClass() {
			return getRuleContext(PitchClassContext.class,0);
		}
		public Chord_qualityContext chord_quality() {
			return getRuleContext(Chord_qualityContext.class,0);
		}
		public Chord_symbolContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_chord_symbol; }
	}

	public final Chord_symbolContext chord_symbol() throws RecognitionException {
		Chord_symbolContext _localctx = new Chord_symbolContext(_ctx, getState());
		enterRule(_localctx, 30, RULE_chord_symbol);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(174);
			pitchClass();
			setState(175);
			chord_quality();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Chord_qualityContext extends ParserRuleContext {
		public TerminalNode ChordQuality() { return getToken(ScoreLangParser.ChordQuality, 0); }
		public List<TerminalNode> Number() { return getTokens(ScoreLangParser.Number); }
		public TerminalNode Number(int i) {
			return getToken(ScoreLangParser.Number, i);
		}
		public List<TerminalNode> ChordAlter() { return getTokens(ScoreLangParser.ChordAlter); }
		public TerminalNode ChordAlter(int i) {
			return getToken(ScoreLangParser.ChordAlter, i);
		}
		public Chord_qualityContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_chord_quality; }
	}

	public final Chord_qualityContext chord_quality() throws RecognitionException {
		Chord_qualityContext _localctx = new Chord_qualityContext(_ctx, getState());
		enterRule(_localctx, 32, RULE_chord_quality);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(177);
			match(ChordQuality);
			setState(179);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==Number) {
				{
				setState(178);
				match(Number);
				}
			}

			setState(185);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==ChordAlter) {
				{
				{
				setState(181);
				match(ChordAlter);
				setState(182);
				match(Number);
				}
				}
				setState(187);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Pitch_listContext extends ParserRuleContext {
		public List<PitchContext> pitch() {
			return getRuleContexts(PitchContext.class);
		}
		public PitchContext pitch(int i) {
			return getRuleContext(PitchContext.class,i);
		}
		public List<TerminalNode> Comma() { return getTokens(ScoreLangParser.Comma); }
		public TerminalNode Comma(int i) {
			return getToken(ScoreLangParser.Comma, i);
		}
		public Pitch_listContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_pitch_list; }
	}

	public final Pitch_listContext pitch_list() throws RecognitionException {
		Pitch_listContext _localctx = new Pitch_listContext(_ctx, getState());
		enterRule(_localctx, 34, RULE_pitch_list);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(188);
			pitch();
			setState(193);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==Comma) {
				{
				{
				setState(189);
				match(Comma);
				setState(190);
				pitch();
				}
				}
				setState(195);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class PitchContext extends ParserRuleContext {
		public PitchClassContext pitchClass() {
			return getRuleContext(PitchClassContext.class,0);
		}
		public TerminalNode Octave() { return getToken(ScoreLangParser.Octave, 0); }
		public PitchContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_pitch; }
	}

	public final PitchContext pitch() throws RecognitionException {
		PitchContext _localctx = new PitchContext(_ctx, getState());
		enterRule(_localctx, 36, RULE_pitch);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(196);
			pitchClass();
			setState(198);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==Octave) {
				{
				setState(197);
				match(Octave);
				}
			}

			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class PitchClassContext extends ParserRuleContext {
		public TerminalNode NoteName() { return getToken(ScoreLangParser.NoteName, 0); }
		public TerminalNode Accidental() { return getToken(ScoreLangParser.Accidental, 0); }
		public PitchClassContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_pitchClass; }
	}

	public final PitchClassContext pitchClass() throws RecognitionException {
		PitchClassContext _localctx = new PitchClassContext(_ctx, getState());
		enterRule(_localctx, 38, RULE_pitchClass);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(200);
			match(NoteName);
			setState(202);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==Accidental) {
				{
				setState(201);
				match(Accidental);
				}
			}

			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class DurationContext extends ParserRuleContext {
		public TerminalNode NoteValue() { return getToken(ScoreLangParser.NoteValue, 0); }
		public TerminalNode For() { return getToken(ScoreLangParser.For, 0); }
		public TerminalNode Dot() { return getToken(ScoreLangParser.Dot, 0); }
		public DurationContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_duration; }
	}

	public final DurationContext duration() throws RecognitionException {
		DurationContext _localctx = new DurationContext(_ctx, getState());
		enterRule(_localctx, 40, RULE_duration);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			{
			setState(204);
			_la = _input.LA(1);
			if ( !(_la==For || _la==Dot) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			setState(205);
			match(NoteValue);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Local_controlContext extends ParserRuleContext {
		public TerminalNode At() { return getToken(ScoreLangParser.At, 0); }
		public Key_controlContext key_control() {
			return getRuleContext(Key_controlContext.class,0);
		}
		public Tempo_controlContext tempo_control() {
			return getRuleContext(Tempo_controlContext.class,0);
		}
		public Time_controlContext time_control() {
			return getRuleContext(Time_controlContext.class,0);
		}
		public Local_controlContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_local_control; }
	}

	public final Local_controlContext local_control() throws RecognitionException {
		Local_controlContext _localctx = new Local_controlContext(_ctx, getState());
		enterRule(_localctx, 42, RULE_local_control);
		try {
			setState(213);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,15,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(207);
				match(At);
				setState(208);
				key_control();
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(209);
				match(At);
				setState(210);
				tempo_control();
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(211);
				match(At);
				setState(212);
				time_control();
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static final String _serializedATN =
		"\u0004\u0001 \u00d8\u0002\u0000\u0007\u0000\u0002\u0001\u0007\u0001\u0002"+
		"\u0002\u0007\u0002\u0002\u0003\u0007\u0003\u0002\u0004\u0007\u0004\u0002"+
		"\u0005\u0007\u0005\u0002\u0006\u0007\u0006\u0002\u0007\u0007\u0007\u0002"+
		"\b\u0007\b\u0002\t\u0007\t\u0002\n\u0007\n\u0002\u000b\u0007\u000b\u0002"+
		"\f\u0007\f\u0002\r\u0007\r\u0002\u000e\u0007\u000e\u0002\u000f\u0007\u000f"+
		"\u0002\u0010\u0007\u0010\u0002\u0011\u0007\u0011\u0002\u0012\u0007\u0012"+
		"\u0002\u0013\u0007\u0013\u0002\u0014\u0007\u0014\u0002\u0015\u0007\u0015"+
		"\u0001\u0000\u0005\u0000.\b\u0000\n\u0000\f\u00001\t\u0000\u0001\u0000"+
		"\u0004\u00004\b\u0000\u000b\u0000\f\u00005\u0001\u0000\u0001\u0000\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0003\u0001B\b\u0001\u0001\u0002\u0001\u0002\u0001"+
		"\u0002\u0001\u0002\u0001\u0002\u0001\u0002\u0001\u0003\u0001\u0003\u0001"+
		"\u0003\u0001\u0003\u0001\u0003\u0001\u0004\u0001\u0004\u0001\u0004\u0001"+
		"\u0004\u0001\u0004\u0001\u0004\u0001\u0004\u0001\u0005\u0001\u0005\u0001"+
		"\u0005\u0001\u0005\u0001\u0005\u0001\u0006\u0001\u0006\u0001\u0006\u0003"+
		"\u0006^\b\u0006\u0001\u0006\u0001\u0006\u0005\u0006b\b\u0006\n\u0006\f"+
		"\u0006e\t\u0006\u0001\u0006\u0001\u0006\u0001\u0007\u0001\u0007\u0001"+
		"\u0007\u0001\u0007\u0001\u0007\u0001\b\u0001\b\u0001\b\u0003\bq\b\b\u0001"+
		"\b\u0001\b\u0005\bu\b\b\n\b\f\bx\t\b\u0001\b\u0001\b\u0001\t\u0001\t\u0001"+
		"\t\u0001\t\u0001\t\u0001\n\u0005\n\u0082\b\n\n\n\f\n\u0085\t\n\u0001\n"+
		"\u0001\n\u0001\u000b\u0001\u000b\u0001\u000b\u0001\u000b\u0003\u000b\u008d"+
		"\b\u000b\u0001\f\u0001\f\u0001\f\u0001\r\u0001\r\u0001\r\u0001\u000e\u0001"+
		"\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0001"+
		"\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0001"+
		"\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0001"+
		"\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0003\u000e\u00ad"+
		"\b\u000e\u0001\u000f\u0001\u000f\u0001\u000f\u0001\u0010\u0001\u0010\u0003"+
		"\u0010\u00b4\b\u0010\u0001\u0010\u0001\u0010\u0005\u0010\u00b8\b\u0010"+
		"\n\u0010\f\u0010\u00bb\t\u0010\u0001\u0011\u0001\u0011\u0001\u0011\u0005"+
		"\u0011\u00c0\b\u0011\n\u0011\f\u0011\u00c3\t\u0011\u0001\u0012\u0001\u0012"+
		"\u0003\u0012\u00c7\b\u0012\u0001\u0013\u0001\u0013\u0003\u0013\u00cb\b"+
		"\u0013\u0001\u0014\u0001\u0014\u0001\u0014\u0001\u0015\u0001\u0015\u0001"+
		"\u0015\u0001\u0015\u0001\u0015\u0001\u0015\u0003\u0015\u00d6\b\u0015\u0001"+
		"\u0015\u0000\u0000\u0016\u0000\u0002\u0004\u0006\b\n\f\u000e\u0010\u0012"+
		"\u0014\u0016\u0018\u001a\u001c\u001e \"$&(*\u0000\u0002\u0001\u0000\u0005"+
		"\u0006\u0001\u0000\u001e\u001f\u00d8\u0000/\u0001\u0000\u0000\u0000\u0002"+
		"A\u0001\u0000\u0000\u0000\u0004C\u0001\u0000\u0000\u0000\u0006I\u0001"+
		"\u0000\u0000\u0000\bN\u0001\u0000\u0000\u0000\nU\u0001\u0000\u0000\u0000"+
		"\fZ\u0001\u0000\u0000\u0000\u000eh\u0001\u0000\u0000\u0000\u0010m\u0001"+
		"\u0000\u0000\u0000\u0012{\u0001\u0000\u0000\u0000\u0014\u0083\u0001\u0000"+
		"\u0000\u0000\u0016\u008c\u0001\u0000\u0000\u0000\u0018\u008e\u0001\u0000"+
		"\u0000\u0000\u001a\u0091\u0001\u0000\u0000\u0000\u001c\u00ac\u0001\u0000"+
		"\u0000\u0000\u001e\u00ae\u0001\u0000\u0000\u0000 \u00b1\u0001\u0000\u0000"+
		"\u0000\"\u00bc\u0001\u0000\u0000\u0000$\u00c4\u0001\u0000\u0000\u0000"+
		"&\u00c8\u0001\u0000\u0000\u0000(\u00cc\u0001\u0000\u0000\u0000*\u00d5"+
		"\u0001\u0000\u0000\u0000,.\u0003\u0002\u0001\u0000-,\u0001\u0000\u0000"+
		"\u0000.1\u0001\u0000\u0000\u0000/-\u0001\u0000\u0000\u0000/0\u0001\u0000"+
		"\u0000\u000003\u0001\u0000\u0000\u00001/\u0001\u0000\u0000\u000024\u0005"+
		"\u0001\u0000\u000032\u0001\u0000\u0000\u000045\u0001\u0000\u0000\u0000"+
		"53\u0001\u0000\u0000\u000056\u0001\u0000\u0000\u000067\u0001\u0000\u0000"+
		"\u000078\u0005\u0000\u0000\u00018\u0001\u0001\u0000\u0000\u00009:\u0005"+
		"\u0002\u0000\u0000:B\u0003\u0004\u0002\u0000;<\u0005\u0002\u0000\u0000"+
		"<B\u0003\u0006\u0003\u0000=>\u0005\u0002\u0000\u0000>B\u0003\b\u0004\u0000"+
		"?@\u0005\u0002\u0000\u0000@B\u0003\n\u0005\u0000A9\u0001\u0000\u0000\u0000"+
		"A;\u0001\u0000\u0000\u0000A=\u0001\u0000\u0000\u0000A?\u0001\u0000\u0000"+
		"\u0000B\u0003\u0001\u0000\u0000\u0000CD\u0005\u0003\u0000\u0000DE\u0005"+
		"\u0004\u0000\u0000EF\u0003&\u0013\u0000FG\u0007\u0000\u0000\u0000GH\u0005"+
		"\u0007\u0000\u0000H\u0005\u0001\u0000\u0000\u0000IJ\u0005\b\u0000\u0000"+
		"JK\u0005\u0004\u0000\u0000KL\u0005\t\u0000\u0000LM\u0005\u0007\u0000\u0000"+
		"M\u0007\u0001\u0000\u0000\u0000NO\u0005\n\u0000\u0000OP\u0005\u0004\u0000"+
		"\u0000PQ\u0005\t\u0000\u0000QR\u0005\u000b\u0000\u0000RS\u0005\t\u0000"+
		"\u0000ST\u0005\u0007\u0000\u0000T\t\u0001\u0000\u0000\u0000UV\u0005\f"+
		"\u0000\u0000VW\u0005\u0004\u0000\u0000WX\u0005\r\u0000\u0000XY\u0005\u0007"+
		"\u0000\u0000Y\u000b\u0001\u0000\u0000\u0000Z[\u0005\u0001\u0000\u0000"+
		"[]\u0005\r\u0000\u0000\\^\u0003\u000e\u0007\u0000]\\\u0001\u0000\u0000"+
		"\u0000]^\u0001\u0000\u0000\u0000^_\u0001\u0000\u0000\u0000_c\u0005\u000e"+
		"\u0000\u0000`b\u0005\u000f\u0000\u0000a`\u0001\u0000\u0000\u0000be\u0001"+
		"\u0000\u0000\u0000ca\u0001\u0000\u0000\u0000cd\u0001\u0000\u0000\u0000"+
		"df\u0001\u0000\u0000\u0000ec\u0001\u0000\u0000\u0000fg\u0005\u0010\u0000"+
		"\u0000g\r\u0001\u0000\u0000\u0000hi\u0005\u0011\u0000\u0000ij\u0005\u0004"+
		"\u0000\u0000jk\u0005\u0012\u0000\u0000kl\u0005\u0007\u0000\u0000l\u000f"+
		"\u0001\u0000\u0000\u0000mn\u0005\u000f\u0000\u0000np\u0005\r\u0000\u0000"+
		"oq\u0003\u0012\t\u0000po\u0001\u0000\u0000\u0000pq\u0001\u0000\u0000\u0000"+
		"qr\u0001\u0000\u0000\u0000rv\u0005\u000e\u0000\u0000su\u0003\u0014\n\u0000"+
		"ts\u0001\u0000\u0000\u0000ux\u0001\u0000\u0000\u0000vt\u0001\u0000\u0000"+
		"\u0000vw\u0001\u0000\u0000\u0000wy\u0001\u0000\u0000\u0000xv\u0001\u0000"+
		"\u0000\u0000yz\u0005\u0010\u0000\u0000z\u0011\u0001\u0000\u0000\u0000"+
		"{|\u0005\u0013\u0000\u0000|}\u0005\u0004\u0000\u0000}~\u0005\t\u0000\u0000"+
		"~\u007f\u0005\u0007\u0000\u0000\u007f\u0013\u0001\u0000\u0000\u0000\u0080"+
		"\u0082\u0003\u0016\u000b\u0000\u0081\u0080\u0001\u0000\u0000\u0000\u0082"+
		"\u0085\u0001\u0000\u0000\u0000\u0083\u0081\u0001\u0000\u0000\u0000\u0083"+
		"\u0084\u0001\u0000\u0000\u0000\u0084\u0086\u0001\u0000\u0000\u0000\u0085"+
		"\u0083\u0001\u0000\u0000\u0000\u0086\u0087\u0005\u0014\u0000\u0000\u0087"+
		"\u0015\u0001\u0000\u0000\u0000\u0088\u008d\u0003\u0018\f\u0000\u0089\u008d"+
		"\u0003\u001a\r\u0000\u008a\u008d\u0003\u001c\u000e\u0000\u008b\u008d\u0003"+
		"*\u0015\u0000\u008c\u0088\u0001\u0000\u0000\u0000\u008c\u0089\u0001\u0000"+
		"\u0000\u0000\u008c\u008a\u0001\u0000\u0000\u0000\u008c\u008b\u0001\u0000"+
		"\u0000\u0000\u008d\u0017\u0001\u0000\u0000\u0000\u008e\u008f\u0003$\u0012"+
		"\u0000\u008f\u0090\u0003(\u0014\u0000\u0090\u0019\u0001\u0000\u0000\u0000"+
		"\u0091\u0092\u0005\u0015\u0000\u0000\u0092\u0093\u0003(\u0014\u0000\u0093"+
		"\u001b\u0001\u0000\u0000\u0000\u0094\u0095\u0005\u0016\u0000\u0000\u0095"+
		"\u0096\u0003\u001e\u000f\u0000\u0096\u0097\u0005\u000b\u0000\u0000\u0097"+
		"\u0098\u0003$\u0012\u0000\u0098\u0099\u0005\u0017\u0000\u0000\u0099\u009a"+
		"\u0003(\u0014\u0000\u009a\u00ad\u0001\u0000\u0000\u0000\u009b\u009c\u0005"+
		"\u0016\u0000\u0000\u009c\u009d\u0003\u001e\u000f\u0000\u009d\u009e\u0005"+
		"\u0017\u0000\u0000\u009e\u009f\u0003(\u0014\u0000\u009f\u00ad\u0001\u0000"+
		"\u0000\u0000\u00a0\u00a1\u0005\u0016\u0000\u0000\u00a1\u00a2\u0003\"\u0011"+
		"\u0000\u00a2\u00a3\u0005\u000b\u0000\u0000\u00a3\u00a4\u0003$\u0012\u0000"+
		"\u00a4\u00a5\u0005\u0017\u0000\u0000\u00a5\u00a6\u0003(\u0014\u0000\u00a6"+
		"\u00ad\u0001\u0000\u0000\u0000\u00a7\u00a8\u0005\u0016\u0000\u0000\u00a8"+
		"\u00a9\u0003\"\u0011\u0000\u00a9\u00aa\u0005\u0017\u0000\u0000\u00aa\u00ab"+
		"\u0003(\u0014\u0000\u00ab\u00ad\u0001\u0000\u0000\u0000\u00ac\u0094\u0001"+
		"\u0000\u0000\u0000\u00ac\u009b\u0001\u0000\u0000\u0000\u00ac\u00a0\u0001"+
		"\u0000\u0000\u0000\u00ac\u00a7\u0001\u0000\u0000\u0000\u00ad\u001d\u0001"+
		"\u0000\u0000\u0000\u00ae\u00af\u0003&\u0013\u0000\u00af\u00b0\u0003 \u0010"+
		"\u0000\u00b0\u001f\u0001\u0000\u0000\u0000\u00b1\u00b3\u0005\u0018\u0000"+
		"\u0000\u00b2\u00b4\u0005\t\u0000\u0000\u00b3\u00b2\u0001\u0000\u0000\u0000"+
		"\u00b3\u00b4\u0001\u0000\u0000\u0000\u00b4\u00b9\u0001\u0000\u0000\u0000"+
		"\u00b5\u00b6\u0005\u0019\u0000\u0000\u00b6\u00b8\u0005\t\u0000\u0000\u00b7"+
		"\u00b5\u0001\u0000\u0000\u0000\u00b8\u00bb\u0001\u0000\u0000\u0000\u00b9"+
		"\u00b7\u0001\u0000\u0000\u0000\u00b9\u00ba\u0001\u0000\u0000\u0000\u00ba"+
		"!\u0001\u0000\u0000\u0000\u00bb\u00b9\u0001\u0000\u0000\u0000\u00bc\u00c1"+
		"\u0003$\u0012\u0000\u00bd\u00be\u0005\u001a\u0000\u0000\u00be\u00c0\u0003"+
		"$\u0012\u0000\u00bf\u00bd\u0001\u0000\u0000\u0000\u00c0\u00c3\u0001\u0000"+
		"\u0000\u0000\u00c1\u00bf\u0001\u0000\u0000\u0000\u00c1\u00c2\u0001\u0000"+
		"\u0000\u0000\u00c2#\u0001\u0000\u0000\u0000\u00c3\u00c1\u0001\u0000\u0000"+
		"\u0000\u00c4\u00c6\u0003&\u0013\u0000\u00c5\u00c7\u0005\u001b\u0000\u0000"+
		"\u00c6\u00c5\u0001\u0000\u0000\u0000\u00c6\u00c7\u0001\u0000\u0000\u0000"+
		"\u00c7%\u0001\u0000\u0000\u0000\u00c8\u00ca\u0005\u001c\u0000\u0000\u00c9"+
		"\u00cb\u0005\u001d\u0000\u0000\u00ca\u00c9\u0001\u0000\u0000\u0000\u00ca"+
		"\u00cb\u0001\u0000\u0000\u0000\u00cb\'\u0001\u0000\u0000\u0000\u00cc\u00cd"+
		"\u0007\u0001\u0000\u0000\u00cd\u00ce\u0005 \u0000\u0000\u00ce)\u0001\u0000"+
		"\u0000\u0000\u00cf\u00d0\u0005\u0002\u0000\u0000\u00d0\u00d6\u0003\u0004"+
		"\u0002\u0000\u00d1\u00d2\u0005\u0002\u0000\u0000\u00d2\u00d6\u0003\u0006"+
		"\u0003\u0000\u00d3\u00d4\u0005\u0002\u0000\u0000\u00d4\u00d6\u0003\b\u0004"+
		"\u0000\u00d5\u00cf\u0001\u0000\u0000\u0000\u00d5\u00d1\u0001\u0000\u0000"+
		"\u0000\u00d5\u00d3\u0001\u0000\u0000\u0000\u00d6+\u0001\u0000\u0000\u0000"+
		"\u0010/5A]cpv\u0083\u008c\u00ac\u00b3\u00b9\u00c1\u00c6\u00ca\u00d5";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}