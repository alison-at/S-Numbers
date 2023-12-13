package main

import (
	"fmt"
	"sync"
	"time"
	"math"
	"os"
	"strconv"
)

type OffsetInfo struct {
	offset int
	num int
}

var (
	mu sync.Mutex
	sFactors []int
	SPerfect []int
	iterating int
	counter int
	currentBar int
	allOffset []OffsetInfo
)

func printFormat() {
	var nums [15][]int

	for idx := range allOffset {
		currentOff := allOffset[idx].offset
		nums[currentOff+7] = append(nums[currentOff+7], allOffset[idx].num)
	}

	fmt.Println("\nDeficient (In S)")
	for x := 0; x <7; x++ {
		fmt.Println(x-7," -->", nums[x])
	}

	fmt.Println("\nPerfect (in S)\n0 --> ", SPerfect)

	fmt.Println("\nAbundant (In S)")
	for y := 8; y < 15; y++ {
		fmt.Println(y-7," -->", nums[y])
	}
}

func factoring(x int) []int{
	var facts []int
	for i := 1; float64(i) <= math.Sqrt(float64(x)); i++ {
		if (x%i == 0) {
			facts = append(facts, i)
			if (x/i != i && i != 1){
				y := x/i;
				facts = append(facts, y)
			}
		}
	}
	return facts
}

func sum(nums []int) int {
	total := 0

	for _,num := range nums {
		total += num
	}
	fmt.Println(total)
	return total
}

func contains(s []int, e int) bool {
    for _,a := range s {
        if a == e {
            return true
        }
    }
    return false
}

func updateSPerfect(newS []int, newSPerfect []int, iteratNum int, newOff []OffsetInfo) {
	defer mu.Unlock()
	mu.Lock()
	sFactors = append(sFactors, newS...)
	SPerfect = append(SPerfect, newSPerfect...)
	allOffset = append(allOffset, newOff...)
	iterating += iteratNum
	counter--
	//fmt.Println("Sperfect", SPerfect,"num S factors", len(sFactors))
	//fmt.Println(sFactors)
}

func iteration(sFactors []int, start int, end int, max int, wg *sync.WaitGroup)  {
	defer wg.Done()

	var newSNums []int
	var newSPer []int
	var newOff []OffsetInfo

	iteratNum := 0
	for i := start; i <= end && i <= max; i++ {
		var currentFa = factoring(i)
		currentSum :=0

		for f := range currentFa{
			if contains(sFactors, currentFa[f]) {
				currentSum += currentFa[f]
			}
		}
		
		if 	currentSum <= i {
			newSNums = append(newSNums, i)
		}

		if (math.Abs(float64( currentSum - i) ) <=7) {
			o := OffsetInfo{offset: currentSum - i, num: i}
			newOff = append(newOff, o)
		}

		if currentSum == i {
			newSPer = append(newSPer, i)
		}
	
	 	iteratNum++;
		//fmt.Println("i", i, "start", start, "end", end, "sum", currentSum)
	}
	updateSPerfect(newSNums, newSPer, iteratNum, newOff)
}

func main() {
	iterating = 2;
	counter = 0;
	sFactors = append(sFactors, 1)
	sFactors = append(sFactors, 2)
	var wg sync.WaitGroup
	args := os.Args
	max,_ := strconv.Atoi(args[1])

	//generate Sfactors and Sperfect up to 12000 (changed to 20000)
	for i := 3; i <= 20000 && i <= max; i++ {
		var currentFa = factoring(i)
		currentSum :=0

		for j := range currentFa{
			if contains(sFactors, currentFa[j]) {
				currentSum += currentFa[j]
			}
		}

		if 	currentSum <= i {
			sFactors = append(sFactors, i)
		}

		if (math.Abs(float64( currentSum - i) ) <=7) {
			o := OffsetInfo{offset: currentSum - i, num: i}
			allOffset = append(allOffset, o)
		}

		if currentSum == i {
			SPerfect = append(SPerfect, i)
		}
		iterating++
	}

	currentBar = iterating + 2
	
	for currentBar < max{
		time.Sleep(1*time.Millisecond)//catch up on counting
		
		for counter < 8 && currentBar < max{
			wg.Add(1)
			counter++
			
			if currentBar+3000 > max {
				go iteration(sFactors, currentBar, max, max, &wg)
				currentBar += (max - currentBar)
				//fmt.Println(counter, "counter", currentBar, "maxVal")
				wg.Wait()
				break;
			} else {
				currentBar +=3000
				go iteration(sFactors, currentBar-2999, currentBar, max, &wg)
			}
			//fmt.Println(counter, "counter", currentBar, "maxVal")
		}
	}

	wg.Wait()
	fmt.Println(SPerfect)
	printFormat()
}