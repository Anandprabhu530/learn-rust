fn main() {
    // let str1 = String::from("abcd");
    // let ans = reverse_prefix(str1,'e');
    let mut vec1 = vec![1,2,3,0,2,2,0,0,0,4];
    movezeros(&mut vec1);
    println!("{:?}",vec1);
  }

//https://leetcode.com/problems/reverse-prefix-of-word/
fn _reverse_prefix(word: String, ch: char) -> String {
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
fn _balanced_string_split(s: String) -> i32 {
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
fn _fizzbuzz(n: i32)-> Vec<String> {
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
fn _twosum(nums:Vec<i32>,target:i32)->Vec<i32>{
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
fn _is_palindrome(x:i32)->bool{
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

//https://leetcode.com/problems/move-zeroes
fn movezeros(nums: &mut Vec<i32>){
    let mut support_vec = Vec::new();
    for i in 0..nums.len(){
        if nums[i]!=0{
            support_vec.push(nums[i]);
        }
    }

    for i in 0..support_vec.len(){
        nums[i] = support_vec[i];
    }

    for i in support_vec.len()..nums.len(){
        nums[i] = 0;
    }
}

//https://leetcode.com/problems/find-pivot-index/
fn _pivot_index(nums:Vec<i32>)->i32{
    let mut rightsum = 0;
    for i in 0..nums.len(){
        rightsum += nums[i];
    }

    let mut leftsum = 0;
    for i in 0..nums.len(){
        rightsum -= nums[i];
        if leftsum==rightsum{
            return i as i32;
        }
        leftsum += nums[i];
    }
    -1
}

//https://leetcode.com/problems/binary-search
fn _binary_search(nums:Vec<i32>,target:i32)->i32{
    if target>nums[nums.len()-1] || target<nums[0]{
        return -1;
    }
    let mut start = 0;
    let mut end = nums.len()-1;
    while start<=end{
        let mid = start + (end-start)/2;
        if nums[mid]==target{
            return mid as _;
        }else if nums[mid]>target{
            end = mid -1;
        }else{
            start = mid + 1;
        }
    }
    -1
}