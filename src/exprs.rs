use std::collections::HashMap;

lazy_static! {
    pub static ref HASHMAP: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("fizzbuzz", vec!["1","2","fizz","4","buzz","fizz","7","8","fizz","buzz","11","fizz","13","14","fizzbuzz"]);
        m.insert("letters-lower", vec!["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"]);
        m.insert("letters-upper", vec!["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"]);
        m.insert("digits", vec!["0","1","2","3","4","5","6","7","8","9"]);
        m.insert("rainbow", vec!["red","orange","yellow","green","blue","indigo","violet"]);
        m.insert("grootify", vec!["I am groot.","I AAAMMM GROOT!!", "I am groot?!", "GROOT!#$%?"]);
        m.insert("planets", vec!["Mercury","Venus","Earth","Mars","Jupiter","Saturn","Uranus","Neptune", "Pluto?"]);
        m.insert("puncutation", vec!["!",",",".","!","\\","/","$","^","#","@","*","(",")","-","+","=","%","#"]);
        m.insert("one-direction", vec!["Niall","Liam","Zayn","Louis","Harry"]);
        m.insert("pizza-toppings", vec!["Pepperoni","Sausage","Peppers","Onions","Black Olives","Pineapple?","Ham",
            "Bacon","Spinich","BBQ Sauce","Chicken","Tomatoes","Basil","Cheese","Hot Sauce"]);
        m.insert("got-quotes",vec![
            "Winter is coming", 
            "Chaos is a ladder",
            "When you play the game of thrones, you win or you die.",
            "The next time you raise a hand to me will be the last time you have hands.",
            "The man who passes the sentence should swing the sword.",
            "Hodor",
            "Power resides where men believe it resides. No more and no less."
            ]);;
        m.insert("alexa", vec![
            "Alexa, order 1000 rolls of paper towels.",
            "Alexa, play Nickelback.",
            "Alexa, sing me a song.",
            "Alexa, how old are you?",
            "Alexa, call grandma.",
            "Alexa, order me a pizza.",
            "Alexa, play despacito."
        ]);
        m.insert("friends-quotes", vec![]);
        m.insert("emojis",vec!["😀", "😃", "😄", "😁", "😆", "😅", "😂", "🤣", "😊", "😇", "🙂", "🙃", "😉", "😌", "😍", "😘",
                              "😗", "😙", "😚", "😋", "😜", "😝", "😛", "🤑", "🤗", "🤓", "😎", "🤡", "🤠", "😏", "😒", "😞", 
                              "😔", "😟", "😕", "🙁", "☹️", "😣", "😖", "😫", "😩", "😤", "😠", "😡", "😶", "😐", "😑", "😯", 
                              "😦", "😧", "😮", "😲", "😵", "😳", "😱", "😨", "😰", "😢", "😥", "🤤", "😭", "😓", "😪", "😴", 
                              "🙄", "🤔", "🤥", "😬", "🤐", "🤢", "🤧", "😷", "🤒", "🤕", "😈", "👿"]);
        m.insert("kittycat-lang",vec!["MEOW", "meow", "mEOw", "Meow", "MeoW", "Meow", "meoW", "prrr", "mrrr"])
        m.insert("dj-khaled-quotes", vec![
            "Call me asparagus",
            "This is called what?",
            "Life... is roblox.",
            "God did!", "DJ KHALED!!!",
            "TELL 'EM BRING OUT THE LOBSTER",
            "Let's go to the beach hahhaha",
            "Let's go swimming hahahah",
            "I call her chandelier",
            "Have you ever played rugby?"])
        m.insert("morse", vec![".", "-", " "])
        m.insert("lol-shaco", vec![
            "This will be fun!",
            "Here we go!",
            "Just a little bit closer!",
            "Now you see me, now you don't!",
            "March, march, march, march!",
            "The joke's on you!",
            "Why so serious?",
            "For my next trick, I'll make you disappear!",
            "Shaco laughs menacingly.",
            "Look... behind you."])
        m
    };
}