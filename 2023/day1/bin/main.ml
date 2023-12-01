
let max x y = 
    if x < y then y else x
;;

let min x y = 
    if x < y then x else y
;;

let take s n = 
    String.sub s 0 (min n (String.length s))
;;

let skip s n = 
    let length = String.length s in
    String.sub s n (length - n)
;;

let string_of_list lst = 
    let rec aux str = function
        | [] -> str
        | hd :: tl -> aux (str ^ " " ^ hd) tl
    in
    "[" ^ aux "" lst ^ " ]"
;;

let rev x =
  let len = String.length x in
  String.init len (fun n -> String.get x (len - n - 1))
;;

let contains l s = 
    let rec aux lst s' = 
        match lst with
        | [] -> false
        | hd :: tl -> if hd = s' then true else aux tl s'
    in
    aux l s
;;

let num_of_word s = 
    let number_words = ["one";"two";"three";"four";"five";"six";"seven";"eight";"nine";"zero"] in
    let i = List.find_index (fun x -> x = s) number_words in
    match i with
        | None -> s
        | Some i -> string_of_int (i + 1)
;;

let find key s = 
    let rg = Str.regexp key in
    let i = try Some (Str.search_forward rg s 0) with Not_found -> None in
    match i with
    | None -> [""]
    | Some i -> [key]
;;

let lex s =
    let number_words = ["1"; "2"; "3"; "4";"5";"6";"7";"8";"9";"one";"two";"three";"four";"five";"six";"seven";"eight";"nine";] in
    let finds = List.map (fun key -> find key s) number_words in
    let finds = List.flatten finds in
    List.filter (fun x -> x <> "") finds
;;

let split_on_nums s =
    let rec aux w_start w_len acc =
        let sub = try Some (String.sub s w_start w_len) with Invalid_argument _ -> None in
        let sub = match sub with
            | None -> ""
            | Some s' -> s'
        in
        let found = lex sub in
        if sub = "" then acc @ found else
        if found = [] then aux w_start (w_len + 1) acc else
        let new_start = if (w_start + w_len - 1) = w_start then w_start + w_len else w_start + w_len - 1 in
        if new_start >= (String.length s) then acc @ found else
        aux new_start 1 (acc @ found)
    in
    aux 0 1 []
;;

let read_file fname = 
    In_channel.with_open_bin fname In_channel.input_all
;;

let rec get_first_digit s =
let nums = split_on_nums s in
    num_of_word (List.nth nums 0)
;;

let get_last_digit s = 
    let nums = split_on_nums s in
    num_of_word (List.nth nums (List.length nums - 1))
;;

let get_num_for_line input =
    let f = get_first_digit input in
    let l = get_last_digit input in
    int_of_string (f ^ l)
;;

let sum_line_nums (input: string): int =
    let lines = String.split_on_char '\n' input in
    let lines = List.filter (fun x -> x <> "") lines in
    let rec aux sum lst = 
        match lst with
        | [] -> sum
        | hd :: tl -> aux (sum + (get_num_for_line hd)) tl
    in
    aux 0 lines
;;


let () = 
    Printexc.record_backtrace true;
    let str = read_file "day1_input.txt" in
    (*let str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
eightwo" in*)
    let sum = sum_line_nums str in
    print_int sum;
    print_endline "";
    ()
;;
