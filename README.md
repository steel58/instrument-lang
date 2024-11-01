# intstrument-lang overview
A interpreted language written as sheet music. While executing the commands the music written is played.

An instrument-lang project file is structured as shown bellow
piece_name
|expression.toml
|instruments
||file1.inst
||file2.inst
||etc.

# Basic Syntax
A bar of music can contain as many instructions or notes as you would like as long as they abide by the time signature set in your expression.toml document. Currently, only treble and bass clefs are available and their declaration is shown below.

## Treble clef declaration:

```
   /\
   | \ | ---------------------------|
   | / |                            |
   |/  | ---------------------------|
  /|   |                            |
 / |   | ---------------------------|
|  |\  |                            | 
 \ | | | ---------------------------|
  \|/  |                            | 
/@ |   | ---------------------------|
\_/
```

## Bass clef declaration:

```
 __
/  \    | ---------------------------|
|   \ @ |                            |
\@@ |   | ---------------------------|
 @@ / @ |                            |
   /    | ---------------------------|
  /     |                            | 
 /      | ---------------------------|
/       |                            | 
        | ---------------------------|
```

You may declare one of each clef in a given instrument file. It is important to remember that the only difference between these is human readablity and pitch when executed.

## Rests
The valid representations of rests in instrument-lang are shown below on the treble clef. Vertical height is irrelevant in the interpreter as long as rests are fully within the top and bottom line of the staff.

```
   /\        thirty-second rest           sixteenth rest                eight rest                  quarter rest         half rest (must be in space) whole rest (must be in space)
   | \ | ---------------------------|---------------------------|---------------------------|---------------------------|---------------------------|---------------------------||
   | / |           @_/              |                           |                           |                           |                           |                           ||
   |/  | ---------@_/---------------|---------@_/---------------|---------@_/---------------|---------------------------|---------------------------|---------------------------||
  /|   |         @_/                |        @_/                |          /                |            Z              |          /===\            |          \===/            ||
 / |   | ---------/-----------------|---------/-----------------|---------/-----------------|------------C--------------|---------------------------|---------------------------||
|  |\  |         /                  |        /                  |                           |                           |                           |                           || 
 \ | | | ---------------------------|---------------------------|---------------------------|---------------------------|---------------------------|---------------------------||
  \|/  |                            |                           |                           |                           |                           |                           || 
/@ |   | ---------------------------|---------------------------|---------------------------|---------------------------|---------------------------|---------------------------||
\_/

```

## Notes
The valid representations of notes in instrument-lang are shown below on the treble clef. Pitch does matter as different pitch combinations correspond to different instructions. Rhythm is simply for the musical sound when executing.

```
   /\        thirty-second note          sixteenth note                eight note                  quarter note                 half note                  whole note 
   | \ | ---------------------------|---------------------------|--------------------|@-----|---------------------------|---------------------------|---------------------------||
   | / |  ____                      |  ____                     |           |\       |      |    |            |@        |     |        |O           |                           ||
   |/  | -|__|-----|\-----|@--------|--|__|---------|\----|@----|--____-----|--------|------|----|------------|---------|-----|--------|------------|---------------------------||
  /|   |  |__|     |\     |         |  |  |         |\    |     |  |  |     |        |      |    |            |         |     |        |            |                           ||
 / |   | -|--|-----|\-----|/--------|--|-@|---------|-----|/----|--|--|-----|--------|/-----|----|------------|---------|-----|--------|------------|------------O--------------||
|  |\  |  | @|    @|      |/        | @|           @|     |/    |  |  |    @|               |   @|            |         |    O|        |            |                           || 
 \ | | | @|---------------|/--------|---------------------------|--|--|---------------------|---------------------------|---------------------------|---------------------------||
  \|/  |                            |                           | @| @|                     |                           |                           |                           || 
/@ |   | ---------------------------|---------------------------|---------------------------|---------------------------|---------------------------|---------------------------||
\_/

```

## Dotting
While these basic rhythms are well and good, one can dot any note or rest to add half of its duration to it's total duration. The dot must be in the column immediately following the note it is dotting. Below is an example in a 4/4 time signature using dotted rhythms. As shown below, height of the "dot" does not matter.
```
   /\        
   | \ | ---------------------------||
   | / |    |                       ||
   |/  | ---|------------______-----||
  /|   |    |    Z       |   _|     ||
 / |   | ---|----C-------|----|-----||
|  |\  |    |            |   @|     || 
 \ | | | --@|.-----------|----------||
  \|/  |          .     @|.         || 
/@ |   | ---------------------------||
\_/         

```

## Triplets
Triplets are valuble to many pieces of music and as you compose you will doubtless feel the need to use triplets. The notation for eight-note triplets to half-note triplets are shown below on the treble clef. As before with dotted rhythms, the height of groupings does not matter. Additionally, the "3" is purely decorative.and is not necessary for interpretation.
```
   /\       eight note triplet          quarter note triplet        half note triplet
   | \ | -----------3---------------|---------------------------|---------------------------||
   | / |        _________           |    |@    |@    |@         |          3                ||
   |/  | -------|---|---|-----------|----|-----|-----|----------|-----|----|---|------------||
  /|   |      < |   |   |           |   <|     |     | >        |     |    |   |            ||
 / |   | -------|---|---|>----------|----|-----|-----|----------|-----|----|---|------------||
|  |\  |       @|  @|  @|           |    |     |     |          |   <O|   O|  O|>           || 
 \ | | | ---------------------------|---------------------------|---------------------------||
  \|/  |                            |          3                |                           || 
/@ |   | ---------------------------|---------------------------|---------------------------||
\_/

```
