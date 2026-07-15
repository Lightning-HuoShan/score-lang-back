// Generated from d:/Program/Projects/ScoreAnalysis/analysis/src/parser/ScoreLang.gram by ANTLR 4.13.1
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
		At=1, KeySet=2, Lpar=3, MAJOR=4, MINOR=5, Rpar=6, TempoSet=7, Number=8, 
		TimeSet=9, Slash=10, TitleSet=11, StringLit=12, Track=13, Lbrace=14, Rbrace=15, 
		Instrument=16, InstList=17, Section=18, Repeat=19, Bar=20, Rest=21, Lbrack=22, 
		Rbrack=23, ChordQuality=24, ChordAlter=25, NoteName=26, Accidental=27, 
		For=28, Dot=29;
	public static final int
		RULE_score = 0, RULE_global = 1, RULE_key_control = 2, RULE_tempo_control = 3, 
		RULE_time_control = 4, RULE_title_control = 5, RULE_track = 6, RULE_instrumentAssign = 7, 
		RULE_section = 8, RULE_repeatAssign = 9, RULE_measure = 10, RULE_event = 11, 
		RULE_note = 12, RULE_rest = 13, RULE_chord = 14, RULE_chord_symbol = 15, 
		RULE_chord_quality = 16, RULE_pitch = 17, RULE_pitchClass = 18, RULE_duration = 19, 
		RULE_local_control = 20;
	private static String[] makeRuleNames() {
		return new String[] {
			"score", "global", "key_control", "tempo_control", "time_control", "title_control", 
			"track", "instrumentAssign", "section", "repeatAssign", "measure", "event", 
			"note", "rest", "chord", "chord_symbol", "chord_quality", "pitch", "pitchClass", 
			"duration", "local_control"
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
			null, "At", "KeySet", "Lpar", "MAJOR", "MINOR", "Rpar", "TempoSet", "Number", 
			"TimeSet", "Slash", "TitleSet", "StringLit", "Track", "Lbrace", "Rbrace", 
			"Instrument", "InstList", "Section", "Repeat", "Bar", "Rest", "Lbrack", 
			"Rbrack", "ChordQuality", "ChordAlter", "NoteName", "Accidental", "For", 
			"Dot"
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
		public List<TrackContext> track() {
			return getRuleContexts(TrackContext.class);
		}
		public TrackContext track(int i) {
			return getRuleContext(TrackContext.class,i);
		}
		public ScoreContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_score; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterScore(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitScore(this);
		}
	}

	public final ScoreContext score() throws RecognitionException {
		ScoreContext _localctx = new ScoreContext(_ctx, getState());
		enterRule(_localctx, 0, RULE_score);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(45);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==At) {
				{
				{
				setState(42);
				global();
				}
				}
				setState(47);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(49); 
			_errHandler.sync(this);
			_la = _input.LA(1);
			do {
				{
				{
				setState(48);
				track();
				}
				}
				setState(51); 
				_errHandler.sync(this);
				_la = _input.LA(1);
			} while ( _la==Track );
			setState(53);
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
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterGlobal(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitGlobal(this);
		}
	}

	public final GlobalContext global() throws RecognitionException {
		GlobalContext _localctx = new GlobalContext(_ctx, getState());
		enterRule(_localctx, 2, RULE_global);
		try {
			setState(63);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,2,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(55);
				match(At);
				setState(56);
				key_control();
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(57);
				match(At);
				setState(58);
				tempo_control();
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(59);
				match(At);
				setState(60);
				time_control();
				}
				break;
			case 4:
				enterOuterAlt(_localctx, 4);
				{
				setState(61);
				match(At);
				setState(62);
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
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterKey_control(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitKey_control(this);
		}
	}

	public final Key_controlContext key_control() throws RecognitionException {
		Key_controlContext _localctx = new Key_controlContext(_ctx, getState());
		enterRule(_localctx, 4, RULE_key_control);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(65);
			match(KeySet);
			setState(66);
			match(Lpar);
			setState(67);
			pitchClass();
			setState(68);
			_la = _input.LA(1);
			if ( !(_la==MAJOR || _la==MINOR) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			setState(69);
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
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterTempo_control(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitTempo_control(this);
		}
	}

	public final Tempo_controlContext tempo_control() throws RecognitionException {
		Tempo_controlContext _localctx = new Tempo_controlContext(_ctx, getState());
		enterRule(_localctx, 6, RULE_tempo_control);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(71);
			match(TempoSet);
			setState(72);
			match(Lpar);
			setState(73);
			match(Number);
			setState(74);
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
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterTime_control(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitTime_control(this);
		}
	}

	public final Time_controlContext time_control() throws RecognitionException {
		Time_controlContext _localctx = new Time_controlContext(_ctx, getState());
		enterRule(_localctx, 8, RULE_time_control);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(76);
			match(TimeSet);
			setState(77);
			match(Lpar);
			setState(78);
			match(Number);
			setState(79);
			match(Slash);
			setState(80);
			match(Number);
			setState(81);
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
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterTitle_control(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitTitle_control(this);
		}
	}

	public final Title_controlContext title_control() throws RecognitionException {
		Title_controlContext _localctx = new Title_controlContext(_ctx, getState());
		enterRule(_localctx, 10, RULE_title_control);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(83);
			match(TitleSet);
			setState(84);
			match(Lpar);
			setState(85);
			match(StringLit);
			setState(86);
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
		public List<SectionContext> section() {
			return getRuleContexts(SectionContext.class);
		}
		public SectionContext section(int i) {
			return getRuleContext(SectionContext.class,i);
		}
		public TrackContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_track; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterTrack(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitTrack(this);
		}
	}

	public final TrackContext track() throws RecognitionException {
		TrackContext _localctx = new TrackContext(_ctx, getState());
		enterRule(_localctx, 12, RULE_track);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(88);
			match(Track);
			setState(89);
			match(StringLit);
			setState(91);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==Instrument) {
				{
				setState(90);
				instrumentAssign();
				}
			}

			setState(93);
			match(Lbrace);
			setState(97);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==Section) {
				{
				{
				setState(94);
				section();
				}
				}
				setState(99);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(100);
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
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterInstrumentAssign(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitInstrumentAssign(this);
		}
	}

	public final InstrumentAssignContext instrumentAssign() throws RecognitionException {
		InstrumentAssignContext _localctx = new InstrumentAssignContext(_ctx, getState());
		enterRule(_localctx, 14, RULE_instrumentAssign);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(102);
			match(Instrument);
			setState(103);
			match(Lpar);
			setState(104);
			match(InstList);
			setState(105);
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
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterSection(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitSection(this);
		}
	}

	public final SectionContext section() throws RecognitionException {
		SectionContext _localctx = new SectionContext(_ctx, getState());
		enterRule(_localctx, 16, RULE_section);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(107);
			match(Section);
			setState(108);
			match(StringLit);
			setState(110);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==Repeat) {
				{
				setState(109);
				repeatAssign();
				}
			}

			setState(112);
			match(Lbrace);
			setState(116);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while ((((_la) & ~0x3f) == 0 && ((1L << _la) & 74448898L) != 0)) {
				{
				{
				setState(113);
				measure();
				}
				}
				setState(118);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(119);
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
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterRepeatAssign(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitRepeatAssign(this);
		}
	}

	public final RepeatAssignContext repeatAssign() throws RecognitionException {
		RepeatAssignContext _localctx = new RepeatAssignContext(_ctx, getState());
		enterRule(_localctx, 18, RULE_repeatAssign);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(121);
			match(Repeat);
			setState(122);
			match(Lpar);
			setState(123);
			match(Number);
			setState(124);
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
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterMeasure(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitMeasure(this);
		}
	}

	public final MeasureContext measure() throws RecognitionException {
		MeasureContext _localctx = new MeasureContext(_ctx, getState());
		enterRule(_localctx, 20, RULE_measure);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(129);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while ((((_la) & ~0x3f) == 0 && ((1L << _la) & 73400322L) != 0)) {
				{
				{
				setState(126);
				event();
				}
				}
				setState(131);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(132);
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
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterEvent(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitEvent(this);
		}
	}

	public final EventContext event() throws RecognitionException {
		EventContext _localctx = new EventContext(_ctx, getState());
		enterRule(_localctx, 22, RULE_event);
		try {
			setState(138);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case NoteName:
				enterOuterAlt(_localctx, 1);
				{
				setState(134);
				note();
				}
				break;
			case Rest:
				enterOuterAlt(_localctx, 2);
				{
				setState(135);
				rest();
				}
				break;
			case Lbrack:
				enterOuterAlt(_localctx, 3);
				{
				setState(136);
				chord();
				}
				break;
			case At:
				enterOuterAlt(_localctx, 4);
				{
				setState(137);
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
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterNote(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitNote(this);
		}
	}

	public final NoteContext note() throws RecognitionException {
		NoteContext _localctx = new NoteContext(_ctx, getState());
		enterRule(_localctx, 24, RULE_note);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(140);
			pitch();
			setState(141);
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
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterRest(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitRest(this);
		}
	}

	public final RestContext rest() throws RecognitionException {
		RestContext _localctx = new RestContext(_ctx, getState());
		enterRule(_localctx, 26, RULE_rest);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(143);
			match(Rest);
			setState(144);
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
		public ChordContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_chord; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterChord(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitChord(this);
		}
	}

	public final ChordContext chord() throws RecognitionException {
		ChordContext _localctx = new ChordContext(_ctx, getState());
		enterRule(_localctx, 28, RULE_chord);
		try {
			setState(158);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,9,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(146);
				match(Lbrack);
				setState(147);
				chord_symbol();
				setState(148);
				match(Slash);
				setState(149);
				pitch();
				setState(150);
				match(Rbrack);
				setState(151);
				duration();
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(153);
				match(Lbrack);
				setState(154);
				chord_symbol();
				setState(155);
				match(Rbrack);
				setState(156);
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
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterChord_symbol(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitChord_symbol(this);
		}
	}

	public final Chord_symbolContext chord_symbol() throws RecognitionException {
		Chord_symbolContext _localctx = new Chord_symbolContext(_ctx, getState());
		enterRule(_localctx, 30, RULE_chord_symbol);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(160);
			pitchClass();
			setState(161);
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
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterChord_quality(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitChord_quality(this);
		}
	}

	public final Chord_qualityContext chord_quality() throws RecognitionException {
		Chord_qualityContext _localctx = new Chord_qualityContext(_ctx, getState());
		enterRule(_localctx, 32, RULE_chord_quality);
		int _la;
		try {
			setState(182);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case ChordQuality:
				enterOuterAlt(_localctx, 1);
				{
				setState(163);
				match(ChordQuality);
				setState(165);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==Number) {
					{
					setState(164);
					match(Number);
					}
				}

				setState(171);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==ChordAlter) {
					{
					{
					setState(167);
					match(ChordAlter);
					setState(168);
					match(Number);
					}
					}
					setState(173);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				}
				break;
			case Number:
				enterOuterAlt(_localctx, 2);
				{
				setState(174);
				match(Number);
				setState(179);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==ChordAlter) {
					{
					{
					setState(175);
					match(ChordAlter);
					setState(176);
					match(Number);
					}
					}
					setState(181);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
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
	public static class PitchContext extends ParserRuleContext {
		public PitchClassContext pitchClass() {
			return getRuleContext(PitchClassContext.class,0);
		}
		public TerminalNode Number() { return getToken(ScoreLangParser.Number, 0); }
		public PitchContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_pitch; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterPitch(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitPitch(this);
		}
	}

	public final PitchContext pitch() throws RecognitionException {
		PitchContext _localctx = new PitchContext(_ctx, getState());
		enterRule(_localctx, 34, RULE_pitch);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(184);
			pitchClass();
			setState(186);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==Number) {
				{
				setState(185);
				match(Number);
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
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterPitchClass(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitPitchClass(this);
		}
	}

	public final PitchClassContext pitchClass() throws RecognitionException {
		PitchClassContext _localctx = new PitchClassContext(_ctx, getState());
		enterRule(_localctx, 36, RULE_pitchClass);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(188);
			match(NoteName);
			setState(190);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==Accidental) {
				{
				setState(189);
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
		public TerminalNode Number() { return getToken(ScoreLangParser.Number, 0); }
		public TerminalNode For() { return getToken(ScoreLangParser.For, 0); }
		public TerminalNode Dot() { return getToken(ScoreLangParser.Dot, 0); }
		public DurationContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_duration; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterDuration(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitDuration(this);
		}
	}

	public final DurationContext duration() throws RecognitionException {
		DurationContext _localctx = new DurationContext(_ctx, getState());
		enterRule(_localctx, 38, RULE_duration);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			{
			setState(192);
			_la = _input.LA(1);
			if ( !(_la==For || _la==Dot) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			setState(193);
			match(Number);
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
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).enterLocal_control(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ScoreLangListener ) ((ScoreLangListener)listener).exitLocal_control(this);
		}
	}

	public final Local_controlContext local_control() throws RecognitionException {
		Local_controlContext _localctx = new Local_controlContext(_ctx, getState());
		enterRule(_localctx, 40, RULE_local_control);
		try {
			setState(201);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,16,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(195);
				match(At);
				setState(196);
				key_control();
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(197);
				match(At);
				setState(198);
				tempo_control();
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(199);
				match(At);
				setState(200);
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
		"\u0004\u0001\u001d\u00cc\u0002\u0000\u0007\u0000\u0002\u0001\u0007\u0001"+
		"\u0002\u0002\u0007\u0002\u0002\u0003\u0007\u0003\u0002\u0004\u0007\u0004"+
		"\u0002\u0005\u0007\u0005\u0002\u0006\u0007\u0006\u0002\u0007\u0007\u0007"+
		"\u0002\b\u0007\b\u0002\t\u0007\t\u0002\n\u0007\n\u0002\u000b\u0007\u000b"+
		"\u0002\f\u0007\f\u0002\r\u0007\r\u0002\u000e\u0007\u000e\u0002\u000f\u0007"+
		"\u000f\u0002\u0010\u0007\u0010\u0002\u0011\u0007\u0011\u0002\u0012\u0007"+
		"\u0012\u0002\u0013\u0007\u0013\u0002\u0014\u0007\u0014\u0001\u0000\u0005"+
		"\u0000,\b\u0000\n\u0000\f\u0000/\t\u0000\u0001\u0000\u0004\u00002\b\u0000"+
		"\u000b\u0000\f\u00003\u0001\u0000\u0001\u0000\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0003\u0001@\b\u0001\u0001\u0002\u0001\u0002\u0001\u0002\u0001\u0002"+
		"\u0001\u0002\u0001\u0002\u0001\u0003\u0001\u0003\u0001\u0003\u0001\u0003"+
		"\u0001\u0003\u0001\u0004\u0001\u0004\u0001\u0004\u0001\u0004\u0001\u0004"+
		"\u0001\u0004\u0001\u0004\u0001\u0005\u0001\u0005\u0001\u0005\u0001\u0005"+
		"\u0001\u0005\u0001\u0006\u0001\u0006\u0001\u0006\u0003\u0006\\\b\u0006"+
		"\u0001\u0006\u0001\u0006\u0005\u0006`\b\u0006\n\u0006\f\u0006c\t\u0006"+
		"\u0001\u0006\u0001\u0006\u0001\u0007\u0001\u0007\u0001\u0007\u0001\u0007"+
		"\u0001\u0007\u0001\b\u0001\b\u0001\b\u0003\bo\b\b\u0001\b\u0001\b\u0005"+
		"\bs\b\b\n\b\f\bv\t\b\u0001\b\u0001\b\u0001\t\u0001\t\u0001\t\u0001\t\u0001"+
		"\t\u0001\n\u0005\n\u0080\b\n\n\n\f\n\u0083\t\n\u0001\n\u0001\n\u0001\u000b"+
		"\u0001\u000b\u0001\u000b\u0001\u000b\u0003\u000b\u008b\b\u000b\u0001\f"+
		"\u0001\f\u0001\f\u0001\r\u0001\r\u0001\r\u0001\u000e\u0001\u000e\u0001"+
		"\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0001"+
		"\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0003\u000e\u009f\b\u000e\u0001"+
		"\u000f\u0001\u000f\u0001\u000f\u0001\u0010\u0001\u0010\u0003\u0010\u00a6"+
		"\b\u0010\u0001\u0010\u0001\u0010\u0005\u0010\u00aa\b\u0010\n\u0010\f\u0010"+
		"\u00ad\t\u0010\u0001\u0010\u0001\u0010\u0001\u0010\u0005\u0010\u00b2\b"+
		"\u0010\n\u0010\f\u0010\u00b5\t\u0010\u0003\u0010\u00b7\b\u0010\u0001\u0011"+
		"\u0001\u0011\u0003\u0011\u00bb\b\u0011\u0001\u0012\u0001\u0012\u0003\u0012"+
		"\u00bf\b\u0012\u0001\u0013\u0001\u0013\u0001\u0013\u0001\u0014\u0001\u0014"+
		"\u0001\u0014\u0001\u0014\u0001\u0014\u0001\u0014\u0003\u0014\u00ca\b\u0014"+
		"\u0001\u0014\u0000\u0000\u0015\u0000\u0002\u0004\u0006\b\n\f\u000e\u0010"+
		"\u0012\u0014\u0016\u0018\u001a\u001c\u001e \"$&(\u0000\u0002\u0001\u0000"+
		"\u0004\u0005\u0001\u0000\u001c\u001d\u00cc\u0000-\u0001\u0000\u0000\u0000"+
		"\u0002?\u0001\u0000\u0000\u0000\u0004A\u0001\u0000\u0000\u0000\u0006G"+
		"\u0001\u0000\u0000\u0000\bL\u0001\u0000\u0000\u0000\nS\u0001\u0000\u0000"+
		"\u0000\fX\u0001\u0000\u0000\u0000\u000ef\u0001\u0000\u0000\u0000\u0010"+
		"k\u0001\u0000\u0000\u0000\u0012y\u0001\u0000\u0000\u0000\u0014\u0081\u0001"+
		"\u0000\u0000\u0000\u0016\u008a\u0001\u0000\u0000\u0000\u0018\u008c\u0001"+
		"\u0000\u0000\u0000\u001a\u008f\u0001\u0000\u0000\u0000\u001c\u009e\u0001"+
		"\u0000\u0000\u0000\u001e\u00a0\u0001\u0000\u0000\u0000 \u00b6\u0001\u0000"+
		"\u0000\u0000\"\u00b8\u0001\u0000\u0000\u0000$\u00bc\u0001\u0000\u0000"+
		"\u0000&\u00c0\u0001\u0000\u0000\u0000(\u00c9\u0001\u0000\u0000\u0000*"+
		",\u0003\u0002\u0001\u0000+*\u0001\u0000\u0000\u0000,/\u0001\u0000\u0000"+
		"\u0000-+\u0001\u0000\u0000\u0000-.\u0001\u0000\u0000\u0000.1\u0001\u0000"+
		"\u0000\u0000/-\u0001\u0000\u0000\u000002\u0003\f\u0006\u000010\u0001\u0000"+
		"\u0000\u000023\u0001\u0000\u0000\u000031\u0001\u0000\u0000\u000034\u0001"+
		"\u0000\u0000\u000045\u0001\u0000\u0000\u000056\u0005\u0000\u0000\u0001"+
		"6\u0001\u0001\u0000\u0000\u000078\u0005\u0001\u0000\u00008@\u0003\u0004"+
		"\u0002\u00009:\u0005\u0001\u0000\u0000:@\u0003\u0006\u0003\u0000;<\u0005"+
		"\u0001\u0000\u0000<@\u0003\b\u0004\u0000=>\u0005\u0001\u0000\u0000>@\u0003"+
		"\n\u0005\u0000?7\u0001\u0000\u0000\u0000?9\u0001\u0000\u0000\u0000?;\u0001"+
		"\u0000\u0000\u0000?=\u0001\u0000\u0000\u0000@\u0003\u0001\u0000\u0000"+
		"\u0000AB\u0005\u0002\u0000\u0000BC\u0005\u0003\u0000\u0000CD\u0003$\u0012"+
		"\u0000DE\u0007\u0000\u0000\u0000EF\u0005\u0006\u0000\u0000F\u0005\u0001"+
		"\u0000\u0000\u0000GH\u0005\u0007\u0000\u0000HI\u0005\u0003\u0000\u0000"+
		"IJ\u0005\b\u0000\u0000JK\u0005\u0006\u0000\u0000K\u0007\u0001\u0000\u0000"+
		"\u0000LM\u0005\t\u0000\u0000MN\u0005\u0003\u0000\u0000NO\u0005\b\u0000"+
		"\u0000OP\u0005\n\u0000\u0000PQ\u0005\b\u0000\u0000QR\u0005\u0006\u0000"+
		"\u0000R\t\u0001\u0000\u0000\u0000ST\u0005\u000b\u0000\u0000TU\u0005\u0003"+
		"\u0000\u0000UV\u0005\f\u0000\u0000VW\u0005\u0006\u0000\u0000W\u000b\u0001"+
		"\u0000\u0000\u0000XY\u0005\r\u0000\u0000Y[\u0005\f\u0000\u0000Z\\\u0003"+
		"\u000e\u0007\u0000[Z\u0001\u0000\u0000\u0000[\\\u0001\u0000\u0000\u0000"+
		"\\]\u0001\u0000\u0000\u0000]a\u0005\u000e\u0000\u0000^`\u0003\u0010\b"+
		"\u0000_^\u0001\u0000\u0000\u0000`c\u0001\u0000\u0000\u0000a_\u0001\u0000"+
		"\u0000\u0000ab\u0001\u0000\u0000\u0000bd\u0001\u0000\u0000\u0000ca\u0001"+
		"\u0000\u0000\u0000de\u0005\u000f\u0000\u0000e\r\u0001\u0000\u0000\u0000"+
		"fg\u0005\u0010\u0000\u0000gh\u0005\u0003\u0000\u0000hi\u0005\u0011\u0000"+
		"\u0000ij\u0005\u0006\u0000\u0000j\u000f\u0001\u0000\u0000\u0000kl\u0005"+
		"\u0012\u0000\u0000ln\u0005\f\u0000\u0000mo\u0003\u0012\t\u0000nm\u0001"+
		"\u0000\u0000\u0000no\u0001\u0000\u0000\u0000op\u0001\u0000\u0000\u0000"+
		"pt\u0005\u000e\u0000\u0000qs\u0003\u0014\n\u0000rq\u0001\u0000\u0000\u0000"+
		"sv\u0001\u0000\u0000\u0000tr\u0001\u0000\u0000\u0000tu\u0001\u0000\u0000"+
		"\u0000uw\u0001\u0000\u0000\u0000vt\u0001\u0000\u0000\u0000wx\u0005\u000f"+
		"\u0000\u0000x\u0011\u0001\u0000\u0000\u0000yz\u0005\u0013\u0000\u0000"+
		"z{\u0005\u0003\u0000\u0000{|\u0005\b\u0000\u0000|}\u0005\u0006\u0000\u0000"+
		"}\u0013\u0001\u0000\u0000\u0000~\u0080\u0003\u0016\u000b\u0000\u007f~"+
		"\u0001\u0000\u0000\u0000\u0080\u0083\u0001\u0000\u0000\u0000\u0081\u007f"+
		"\u0001\u0000\u0000\u0000\u0081\u0082\u0001\u0000\u0000\u0000\u0082\u0084"+
		"\u0001\u0000\u0000\u0000\u0083\u0081\u0001\u0000\u0000\u0000\u0084\u0085"+
		"\u0005\u0014\u0000\u0000\u0085\u0015\u0001\u0000\u0000\u0000\u0086\u008b"+
		"\u0003\u0018\f\u0000\u0087\u008b\u0003\u001a\r\u0000\u0088\u008b\u0003"+
		"\u001c\u000e\u0000\u0089\u008b\u0003(\u0014\u0000\u008a\u0086\u0001\u0000"+
		"\u0000\u0000\u008a\u0087\u0001\u0000\u0000\u0000\u008a\u0088\u0001\u0000"+
		"\u0000\u0000\u008a\u0089\u0001\u0000\u0000\u0000\u008b\u0017\u0001\u0000"+
		"\u0000\u0000\u008c\u008d\u0003\"\u0011\u0000\u008d\u008e\u0003&\u0013"+
		"\u0000\u008e\u0019\u0001\u0000\u0000\u0000\u008f\u0090\u0005\u0015\u0000"+
		"\u0000\u0090\u0091\u0003&\u0013\u0000\u0091\u001b\u0001\u0000\u0000\u0000"+
		"\u0092\u0093\u0005\u0016\u0000\u0000\u0093\u0094\u0003\u001e\u000f\u0000"+
		"\u0094\u0095\u0005\n\u0000\u0000\u0095\u0096\u0003\"\u0011\u0000\u0096"+
		"\u0097\u0005\u0017\u0000\u0000\u0097\u0098\u0003&\u0013\u0000\u0098\u009f"+
		"\u0001\u0000\u0000\u0000\u0099\u009a\u0005\u0016\u0000\u0000\u009a\u009b"+
		"\u0003\u001e\u000f\u0000\u009b\u009c\u0005\u0017\u0000\u0000\u009c\u009d"+
		"\u0003&\u0013\u0000\u009d\u009f\u0001\u0000\u0000\u0000\u009e\u0092\u0001"+
		"\u0000\u0000\u0000\u009e\u0099\u0001\u0000\u0000\u0000\u009f\u001d\u0001"+
		"\u0000\u0000\u0000\u00a0\u00a1\u0003$\u0012\u0000\u00a1\u00a2\u0003 \u0010"+
		"\u0000\u00a2\u001f\u0001\u0000\u0000\u0000\u00a3\u00a5\u0005\u0018\u0000"+
		"\u0000\u00a4\u00a6\u0005\b\u0000\u0000\u00a5\u00a4\u0001\u0000\u0000\u0000"+
		"\u00a5\u00a6\u0001\u0000\u0000\u0000\u00a6\u00ab\u0001\u0000\u0000\u0000"+
		"\u00a7\u00a8\u0005\u0019\u0000\u0000\u00a8\u00aa\u0005\b\u0000\u0000\u00a9"+
		"\u00a7\u0001\u0000\u0000\u0000\u00aa\u00ad\u0001\u0000\u0000\u0000\u00ab"+
		"\u00a9\u0001\u0000\u0000\u0000\u00ab\u00ac\u0001\u0000\u0000\u0000\u00ac"+
		"\u00b7\u0001\u0000\u0000\u0000\u00ad\u00ab\u0001\u0000\u0000\u0000\u00ae"+
		"\u00b3\u0005\b\u0000\u0000\u00af\u00b0\u0005\u0019\u0000\u0000\u00b0\u00b2"+
		"\u0005\b\u0000\u0000\u00b1\u00af\u0001\u0000\u0000\u0000\u00b2\u00b5\u0001"+
		"\u0000\u0000\u0000\u00b3\u00b1\u0001\u0000\u0000\u0000\u00b3\u00b4\u0001"+
		"\u0000\u0000\u0000\u00b4\u00b7\u0001\u0000\u0000\u0000\u00b5\u00b3\u0001"+
		"\u0000\u0000\u0000\u00b6\u00a3\u0001\u0000\u0000\u0000\u00b6\u00ae\u0001"+
		"\u0000\u0000\u0000\u00b7!\u0001\u0000\u0000\u0000\u00b8\u00ba\u0003$\u0012"+
		"\u0000\u00b9\u00bb\u0005\b\u0000\u0000\u00ba\u00b9\u0001\u0000\u0000\u0000"+
		"\u00ba\u00bb\u0001\u0000\u0000\u0000\u00bb#\u0001\u0000\u0000\u0000\u00bc"+
		"\u00be\u0005\u001a\u0000\u0000\u00bd\u00bf\u0005\u001b\u0000\u0000\u00be"+
		"\u00bd\u0001\u0000\u0000\u0000\u00be\u00bf\u0001\u0000\u0000\u0000\u00bf"+
		"%\u0001\u0000\u0000\u0000\u00c0\u00c1\u0007\u0001\u0000\u0000\u00c1\u00c2"+
		"\u0005\b\u0000\u0000\u00c2\'\u0001\u0000\u0000\u0000\u00c3\u00c4\u0005"+
		"\u0001\u0000\u0000\u00c4\u00ca\u0003\u0004\u0002\u0000\u00c5\u00c6\u0005"+
		"\u0001\u0000\u0000\u00c6\u00ca\u0003\u0006\u0003\u0000\u00c7\u00c8\u0005"+
		"\u0001\u0000\u0000\u00c8\u00ca\u0003\b\u0004\u0000\u00c9\u00c3\u0001\u0000"+
		"\u0000\u0000\u00c9\u00c5\u0001\u0000\u0000\u0000\u00c9\u00c7\u0001\u0000"+
		"\u0000\u0000\u00ca)\u0001\u0000\u0000\u0000\u0011-3?[ant\u0081\u008a\u009e"+
		"\u00a5\u00ab\u00b3\u00b6\u00ba\u00be\u00c9";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}