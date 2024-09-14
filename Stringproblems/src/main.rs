fn main() {
    // let str1 = String::from("abcd");
    // let ans = reverse_prefix(str1,'e');
    let ans = fizzbuzz(15);
    println!("{:?}",ans);
  }

//https://leetcode.com/problems/reverse-prefix-of-word/
fn reverse_prefix(word: String, ch: char) -> String {
    let mut ans = String::new();
    let mut first = false;
    for letter in word.chars(){
        ans.push_str(&String::from(letter));
        if letter==ch && first==false {
            first = true;
            ans = ans.chars().rev().collect::<String>();
        }
    }
    ans
}

//https://leetcode.com/problems/truncate-sentence/description/
fn _truncate_sentence(s: String, k: i32) -> String {
    let mut a = 0;
    let mut ans = String::new();
    for words in s.split_whitespace(){
        ans.push_str(&String::from(words));
        a+=1;
        if a>=k{
            return ans;
        }
        ans.push_str(&String::from(" "));
    }
    ans
}

//https://leetcode.com/problems/split-a-string-in-balanced-strings/description/
fn balanced_string_split(s: String) -> i32 {
    let mut l =0;
    let mut ans =0;
    for word in s.chars(){
        if word=='L'{
            l+=1;
        }else{
            l-=1;
        }
        if l==0{
            ans+=1;
        }
    }
    
    ans
}

//G()(a)l --> Leetcode
fn _interpret(command: String) -> String {
    let mut ans = String::from("");
    let mut check = false;

    for word in command.chars(){
        if check==true && word==')' {
            ans.push_str("o");
        }
        if word == '('{
            check = true;
        }else if word==')'{
            check = false;
        }else{
            check = false;
            ans.push_str(&String::from(word));
        }
    }
    ans
}

fn fizzbuzz(n: i32)-> Vec<String> {
    let mut ans = Vec::new();
    for i in 1..n+1{
        if i%5==0 && i%3==0{
            ans.push("FizzBuzz".to_string());
         }else if i%3==0{
            ans.push("Fizz".to_string());
        }else if i%5==0{
               ans.push("Buzz".to_string());
        }else{
            ans.push(i.to_string());
        }
    }
    ans
}