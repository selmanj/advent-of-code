(ns advent_of_code.day01 
  (:require [clojure.java.io :as io]
            [clojure.string :as str]))

(defn input [] 
  (-> "day01-input.txt"
      io/resource
      io/file
      slurp
      str/split-lines))

(defn freqs []
  (->> (input)
       (map #(Integer/parseInt %))))

(defn answer-one []
  (reduce + (freqs)))

(defn freq-cycle []
  (cycle (freqs)))

(defn answer-two []
  (loop [fs (freq-cycle)
         accum 0
         visited #{}]
      (let [next-accum (+ accum (first fs))]
        (if (get visited next-accum)
          next-accum
          (recur (rest fs) 
                 next-accum 
                 (conj visited next-accum))))))