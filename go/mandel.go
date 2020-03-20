package main

import (
	"fmt"
	"image"
	"image/color"
	"image/png"
	"math/cmplx"
	"os"
	"runtime"
)

//alias
var print = fmt.Println
var write = fmt.Printf

//global consts
const fineness int = 5000
const maxIters int = 1000

type pixel struct {
	x int
	y int
}

type mandelInput struct {
	p pixel
	c complex128
}

type mandelOutput struct {
	p          pixel
	iterations int
}

func main() {
	var center [2]float64 = [2]float64{-0.25, 0.65}
	width := 0.025
	// var center [2]float64 = [2]float64{-0.5, 0}
	// width := 2.0
	var pointsComputed [fineness][fineness]int

	step := width / float64(fineness)
	left := center[0] - width/2
	right := center[0] + width/2
	top := center[1] + width/2
	bottom := center[1] - width/2

	//make threads
	numThreads := runtime.NumCPU()
	runtime.GOMAXPROCS(numThreads)
	jobs := make(chan mandelInput)
	results := make(chan mandelOutput)
	for i := 0; i < numThreads; i++ {
		go worker(i, jobs, results, step, bottom, top)
	}

	//pool results
	go func() {
		for val := range results {
			pointsComputed[val.p.y][val.p.x] = val.iterations
		}
	}()

	//make jobs
	i := 0
	j := 0
	re := left
	im := top
	for re <= right && i < fineness {
		jobs <- mandelInput{pixel{i, j}, complex(re, im)}
		re += step
		i++
	}
	close(jobs)
	print("done computing")

	//save results as image
	saveToPNG(pointsComputed, maxIters, "mandelbrot.png")
}

func step(z complex128, c complex128) complex128 {
	return z*z + c
}
func iterate(c complex128, max_iters int) int {
	var z complex128 = 0
	for i := 0; i < max_iters; i++ {
		if cmplx.Abs(z) >= 2 {
			return i
		}
		z = step(z, c)
	}
	return max_iters
}
func worker(id int, jobs <-chan mandelInput, results chan<- mandelOutput, step float64, bottom float64, top float64) {
	for job := range jobs {
		i := job.p.x
		re := real(job.c)
		j := 0
		im := top
		for im >= bottom && j < fineness {
			results <- mandelOutput{pixel{i, j}, iterate(complex(re, im), maxIters)}
			im -= step
			j++
		}
	}
}

func saveToPNG(pixels [fineness][fineness]int, maxIters int, filename string) {
	myImage := image.NewGray(image.Rect(0, 0, fineness, fineness))
	for y := range pixels {
		for x := range pixels[y] {
			val := pixels[y][x] % 255
			if pixels[y][x] == maxIters {
				val = 0
			}
			myImage.Set(y, x, color.Gray{uint8(val)})
		}
	}
	outputFile, _ := os.Create(filename)
	png.Encode(outputFile, myImage)
	outputFile.Close()
}
