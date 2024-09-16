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

//https://leetcode.com/problems/fizz-buzz/description/
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

// https://leetcode.com/problems/two-sum/
//Improve fix with hashmap
fn twosum(nums:Vec<i32>,target:i32)->Vec<i32>{
    let mut ans = Vec::new();
    for i in 0..nums.len(){
        for j in i+1..nums.len(){
            if &nums[i]+&nums[j]==target{
                ans.push(i as i32);
                ans.push(j as i32);
                return ans;
            }
        }
    }
    ans
}

// https://leetcode.com/problems/palindrome-number/
//optimize
fn is_palindrome(x:i32)->bool{
    // if x<0{
    //     return false;
    // }
    // let mut temp:i32 = x.clone();
    // let mut rev:i32 = 0;
    // while temp!=0{
    //     let ans = temp%10;
    //     rev = ans + rev*10;
    //     temp = temp/10;
    // }
    // if x!=rev{
    //     return false;
    // }
    // true
    let ans = x.to_string();
    for index in 0..ans.len()/2{
        if ans.chars().nth(index).unwrap() != ans.chars().nth(ans.len()-1-index).unwrap(){
            return false;
        }

    }
    true
    //one liner
    // return x.to_string().chars().rev().eq(x.to_string().chars());
}

//https://leetcode.com/problems/search-insert-position/
//runtime error change code
fn _search_insert()->i32{
    let nums = vec![1,3,5,6];
    let target = 5;
    let mut start = 0;
    let mut end = nums.len()-1;
    while start<=end{
        let mid = start + (end-start)/2;
        if &nums[mid]==&target{
            return mid as i32;
        }else if &nums[mid]>&target{
            end = mid-1;
        }else{
            start = mid +1;
        }
    }
    return start as i32;
}