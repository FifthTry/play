module Realm.Trace exposing (..)

import Json.Decode as JD
import Json.Encode as JE
import Time


type alias Trace =
    { id : String
    , createdOn : Time.Posix
    , first : Span
    }


trace : JD.Decoder Trace
trace =
    JD.map3 Trace
        (JD.field "id" JD.string)
        (JD.field "created_on" (JD.map Time.millisToPosix JD.int))
        -- TODO: decoder should fail if list has more than one element
        (JD.field "span_stack" (JD.index 0 span))


type Duration
    = Duration Int


durationToInt : Duration -> Int
durationToInt (Duration d) =
    d


duration : JD.Decoder Duration
duration =
    JD.map Duration JD.int


type SpanItem
    = Log String
    | Field String JE.Value
    | Frame Span
    | TransientField String JE.Value
    | QueryI Query


type alias Query =
    { query : String
    , bind : Maybe String

    -- TODO: result is missing
    , result : Result String Int
    }


result : JD.Decoder e -> JD.Decoder s -> JD.Decoder (Result e s)
result ed sd =
    JD.oneOf [ JD.field "Err" (JD.map Err ed), JD.field "Ok" (JD.map Ok sd) ]


query : JD.Decoder Query
query =
    JD.map3 Query
        (JD.field "query" JD.string)
        (JD.field "bind" (JD.maybe JD.string))
        (JD.field "result" (result JD.string JD.int))


spanItem : JD.Decoder SpanItem
spanItem =
    JD.field "type" JD.string
        |> JD.andThen
            (\tag ->
                case tag of
                    "Log" ->
                        JD.map Log
                            (JD.field "message" JD.string)

                    "Field" ->
                        JD.map2 Field
                            (JD.field "name" JD.string)
                            (JD.field "value" JD.value)

                    "TransientField" ->
                        JD.map2 TransientField
                            (JD.field "name" JD.string)
                            (JD.field "value" JD.value)

                    "Frame" ->
                        JD.map Frame span

                    "Query" ->
                        JD.map QueryI query

                    _ ->
                        JD.fail ("unknown type: " ++ tag)
            )


type Span
    = Span
        { id : String
        , duration : Duration
        , items : List ( Duration, SpanItem )
        }


id : Span -> String
id (Span s) =
    s.id


items : Span -> List ( Duration, SpanItem )
items (Span s) =
    s.items


human : Bool -> Duration -> String
human compact dur =
    let
        d =
            durationToInt dur

        ( secs, nanos ) =
            ( d // milli, remainderBy milli d )

        kilo =
            1000

        micro =
            kilo * kilo

        milli =
            micro * kilo

        nanosPart =
            if nanos < kilo then
                String.fromInt d ++ "ns"

            else if nanos < micro then
                String.fromInt (d // kilo) ++ "??s"

            else
                String.fromInt (nanos // micro)
                    ++ (if compact then
                            ""

                        else
                            "ms"
                       )
    in
    if secs == 0 then
        nanosPart

    else
        String.fromInt secs ++ "s" ++ nanosPart


spanDuration : Bool -> Span -> String
spanDuration compact (Span s) =
    human compact s.duration


tuple : JD.Decoder a -> JD.Decoder b -> JD.Decoder ( a, b )
tuple a b =
    JD.map2 Tuple.pair (JD.index 0 a) (JD.index 1 b)


span : JD.Decoder Span
span =
    JD.map3 (\i d it -> Span { id = i, duration = d, items = it })
        (JD.field "id" JD.string)
        (JD.field "duration" duration)
        (JD.field "items" (JD.list (tuple duration spanItem)))
