data <- read.csv(file.path("data.csv"), sep=";",header = TRUE)

quantile(data[, 2], type=7)
# TODO: can I export this as a function?
# 1
x = data$oszczednosci
y = round(table(x)/length(x)*100, digits=2)[c(1,4,3,2,5)]
barplot(y, col = "#123456" , main = "Procent dochodu przeznaczony na oszczędności" , xlab = "Procent dochodu" , ylab = "Procent osób" , names.arg = c("0%","< 5%","5 - 10%","10 - 15%",">15%"))
dev.print(png, '1.png', height=500, width=500)

# 2
x = data$oszczednosci[data$plec == "K"]
y = round(table(x)/length(x)*100, digits=2)[c(1,4,3,2,5)]
barplot(y, col = "#123456" , main = "Procent dochodu wśród kobiet przeznaczony na oszczędności" , xlab = "Procent dochodu" , ylab = "Procent kobiet" , names.arg = c("0%","< 5%","5 - 10%","10 - 15%",">15%"))
dev.print(png, '2K.png', height=500, width=500)

x = data$oszczednosci[data$plec == "M"]
y = round(table(x)/length(x)*100, digits=2)[c(1,4,3,2,5)]
barplot(y, col = "#123456" , main = "Procent dochodu wśród mężczyzn przeznaczony na oszczędności" , xlab = "Procent dochodu" , ylab = "Procent mężczyzn" , names.arg = c("0%","< 5%","5 - 10%","10 - 15%",">15%"))
dev.print(png, '2M.png', height=500, width=500)

# 3
x = data$wynagrodzenie
hist(x, freq = TRUE, breaks = seq(from = min(x), to = max(x), by = (max(x)-min(x))/9) , right = FALSE , include.lowest = TRUE , main = "Histogram wynagrodzenia" , xlab = "Wynagrodzenie [tys. PLN]" , ylab = "" , col = "#696969")
dev.print(png, '3.png', height=500, width=500)

# 4
x = data$wynagrodzenie[data$plec == "K"]
h = hist(x, freq = TRUE, breaks = seq(from = min(x), to = max(x), by = (max(x)-min(x))/9) , right = FALSE , include.lowest = TRUE , main = "Histogram wynagrodzenia kobiet" , xlab = "Wynagrodzenie [tys. PLN]" , ylab = "" , col = "#696969")
dev.print(png, '4K.png', height=500, width=500)


x = data$wynagrodzenie[data$plec == "M"]
h = hist(x, freq = TRUE, breaks = seq(from = min(x), to = max(x), by = (max(x)-min(x))/9) , right = FALSE , include.lowest = TRUE , main = "Histogram wynagrodzenia mężczyzn" , xlab = "Wynagrodzenie [tys. PLN]" , ylab = "" , col = "#696969")
dev.print(png, '4M.png', height=500, width=500)

# 5
# TODO: how can I rearange the output?
table(data$zamieszkanie,data$wiedza)

# 6
x = split(x = data$wynagrodzenie, f = list(data$zamieszkanie, data$wyksztalcenie))
lapply(x, FUN = mean)
