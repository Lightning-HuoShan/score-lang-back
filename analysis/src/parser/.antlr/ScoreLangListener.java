// Generated from d:/Program/Projects/ScoreAnalysis/analysis/src/parser/ScoreLang.gram by ANTLR 4.13.1
import org.antlr.v4.runtime.tree.ParseTreeListener;

/**
 * This interface defines a complete listener for a parse tree produced by
 * {@link ScoreLangParser}.
 */
public interface ScoreLangListener extends ParseTreeListener {
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#score}.
	 * @param ctx the parse tree
	 */
	void enterScore(ScoreLangParser.ScoreContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#score}.
	 * @param ctx the parse tree
	 */
	void exitScore(ScoreLangParser.ScoreContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#global}.
	 * @param ctx the parse tree
	 */
	void enterGlobal(ScoreLangParser.GlobalContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#global}.
	 * @param ctx the parse tree
	 */
	void exitGlobal(ScoreLangParser.GlobalContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#key_control}.
	 * @param ctx the parse tree
	 */
	void enterKey_control(ScoreLangParser.Key_controlContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#key_control}.
	 * @param ctx the parse tree
	 */
	void exitKey_control(ScoreLangParser.Key_controlContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#tempo_control}.
	 * @param ctx the parse tree
	 */
	void enterTempo_control(ScoreLangParser.Tempo_controlContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#tempo_control}.
	 * @param ctx the parse tree
	 */
	void exitTempo_control(ScoreLangParser.Tempo_controlContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#time_control}.
	 * @param ctx the parse tree
	 */
	void enterTime_control(ScoreLangParser.Time_controlContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#time_control}.
	 * @param ctx the parse tree
	 */
	void exitTime_control(ScoreLangParser.Time_controlContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#title_control}.
	 * @param ctx the parse tree
	 */
	void enterTitle_control(ScoreLangParser.Title_controlContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#title_control}.
	 * @param ctx the parse tree
	 */
	void exitTitle_control(ScoreLangParser.Title_controlContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#track}.
	 * @param ctx the parse tree
	 */
	void enterTrack(ScoreLangParser.TrackContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#track}.
	 * @param ctx the parse tree
	 */
	void exitTrack(ScoreLangParser.TrackContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#instrumentAssign}.
	 * @param ctx the parse tree
	 */
	void enterInstrumentAssign(ScoreLangParser.InstrumentAssignContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#instrumentAssign}.
	 * @param ctx the parse tree
	 */
	void exitInstrumentAssign(ScoreLangParser.InstrumentAssignContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#section}.
	 * @param ctx the parse tree
	 */
	void enterSection(ScoreLangParser.SectionContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#section}.
	 * @param ctx the parse tree
	 */
	void exitSection(ScoreLangParser.SectionContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#repeatAssign}.
	 * @param ctx the parse tree
	 */
	void enterRepeatAssign(ScoreLangParser.RepeatAssignContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#repeatAssign}.
	 * @param ctx the parse tree
	 */
	void exitRepeatAssign(ScoreLangParser.RepeatAssignContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#measure}.
	 * @param ctx the parse tree
	 */
	void enterMeasure(ScoreLangParser.MeasureContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#measure}.
	 * @param ctx the parse tree
	 */
	void exitMeasure(ScoreLangParser.MeasureContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#event}.
	 * @param ctx the parse tree
	 */
	void enterEvent(ScoreLangParser.EventContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#event}.
	 * @param ctx the parse tree
	 */
	void exitEvent(ScoreLangParser.EventContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#note}.
	 * @param ctx the parse tree
	 */
	void enterNote(ScoreLangParser.NoteContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#note}.
	 * @param ctx the parse tree
	 */
	void exitNote(ScoreLangParser.NoteContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#rest}.
	 * @param ctx the parse tree
	 */
	void enterRest(ScoreLangParser.RestContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#rest}.
	 * @param ctx the parse tree
	 */
	void exitRest(ScoreLangParser.RestContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#chord}.
	 * @param ctx the parse tree
	 */
	void enterChord(ScoreLangParser.ChordContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#chord}.
	 * @param ctx the parse tree
	 */
	void exitChord(ScoreLangParser.ChordContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#chord_symbol}.
	 * @param ctx the parse tree
	 */
	void enterChord_symbol(ScoreLangParser.Chord_symbolContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#chord_symbol}.
	 * @param ctx the parse tree
	 */
	void exitChord_symbol(ScoreLangParser.Chord_symbolContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#chord_quality}.
	 * @param ctx the parse tree
	 */
	void enterChord_quality(ScoreLangParser.Chord_qualityContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#chord_quality}.
	 * @param ctx the parse tree
	 */
	void exitChord_quality(ScoreLangParser.Chord_qualityContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#pitch}.
	 * @param ctx the parse tree
	 */
	void enterPitch(ScoreLangParser.PitchContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#pitch}.
	 * @param ctx the parse tree
	 */
	void exitPitch(ScoreLangParser.PitchContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#pitchClass}.
	 * @param ctx the parse tree
	 */
	void enterPitchClass(ScoreLangParser.PitchClassContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#pitchClass}.
	 * @param ctx the parse tree
	 */
	void exitPitchClass(ScoreLangParser.PitchClassContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#duration}.
	 * @param ctx the parse tree
	 */
	void enterDuration(ScoreLangParser.DurationContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#duration}.
	 * @param ctx the parse tree
	 */
	void exitDuration(ScoreLangParser.DurationContext ctx);
	/**
	 * Enter a parse tree produced by {@link ScoreLangParser#local_control}.
	 * @param ctx the parse tree
	 */
	void enterLocal_control(ScoreLangParser.Local_controlContext ctx);
	/**
	 * Exit a parse tree produced by {@link ScoreLangParser#local_control}.
	 * @param ctx the parse tree
	 */
	void exitLocal_control(ScoreLangParser.Local_controlContext ctx);
}