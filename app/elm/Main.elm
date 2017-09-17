module Main exposing (..)

import Html exposing (..)
import Html.Attributes exposing (class, target, href, property, defaultValue)
import Html.Events exposing (..)
import Json.Decode exposing (..)
import Json.Decode.Pipeline exposing (..)

sampleJson : String
sampleJson =
    """
     {
       "results" : [
         {
           "id": 92,
           "name": "Harold",
           "avatar": "avatar",
           "department": "Harris Teeter"
         }
       ]
     }
    """

main : Program Never Model Msg
main =
    Html.beginnerProgram
        { view = view
        , update = update
        , model = initialModel
        }


type alias Robot =
    { id : Int
    , name : String
    , avatar : String
    , department : String
    }

searchResultDecoder : Decoder Robot
searchResultDecoder =
    decode Robot
        |> required "id" int
        |> required "name" string
        |> required "avatar" string
        |> required "department" string


type alias Model =
    { query : String
    , results : List Robot
    }

initialModel : Model
initialModel =
    { query = "Name of Robot"
    , results = decodeResults sampleJson
    }


responseDecoder : Decoder (List Robot)
responseDecoder =
    decode identity
        |> required "results" (list searchResultDecoder)


decodeResults : String -> List Robot
decodeResults json =
    case decodeString responseDecoder json of
        Ok searchResults ->
            searchResults
        Err errorMessage ->
            []


view : Model -> Html Msg
view model =
    div [ class "content" ]
        [ header []
            [ h1 [] [ text "RobotWorld Terraformed" ]
            , span [ class "tagline" ] [ text "Rust + Rocket + Elm" ]
            ]
        , input [ class "search-query", onInput SetQuery, defaultValue model.query ] []
        , button [ class "search-button" ] [ text "Search" ]
        , ul [ class "results" ]
            (List.map viewSearchResult model.results)
        , button [ class "reset-button", onClick ResetResults ] [ text "Reset" ]
        ]

viewSearchResult : Robot -> Html Msg
viewSearchResult result =
    li [ class "robot" ]
        [ span [ class "robot-id" ] [ text (toString result.id) ]
        , a [] [ text result.name ]
        , div [] [ text result.avatar ]
        , div [] [ text result.department]
        , button [ class "hide-result", onClick (DeleteById result.id) ]
            [ text "X" ]
        ]


type Msg
    = SetQuery String
    | DeleteById Int
    | ResetResults


update : Msg -> Model -> Model
update msg model =
    case msg of
        SetQuery query ->
            { model | query = query }

        DeleteById idToHide ->
            let
                newResults =
                    List.filter (\{ id } -> id /= idToHide) model.results
            in
                { model | results = newResults }
        ResetResults ->
            { model | results = decodeResults sampleJson }
