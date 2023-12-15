

library(tidyverse)
library(raster)
library(scico)


setwd("D:/OneDrive/Projects/asf_bavaria/Prototype/asf_model_bav_prototype")

grid <- read.csv("./output/all_grid_states.csv", header = T)

individuals <- read.csv("./output/all_individuals.csv", header = T)

globals <- read.csv("./output/all_global_variables.csv", header = T)


g1 <- grid #%>% filter(iteration == 1)

i1 <- individuals %>% filter(iteration == 2500)

g2 <- g1 %>% mutate(quality = case_when(quality == -9999 ~ 0))

g2 <- g1 %>% mutate(quality = replace(quality, quality<0, NA))

(p1<-ggplot(g2)+
  geom_tile(aes(x=x, y=y, fill = quality))+
  scale_fill_gradient(low = "#c2e9cf", high = "#046104") +
  geom_point(data= i1,aes(x=x, y=y, color = 'red'), size = 3)+#, position=position_dodge(width=1))+
  scale_x_reverse()+
  coord_flip()
)


g3 <- g2 %>% filter(ap == "true") #mutate(is_ap =case_when(ap == "true" ~ 1, FALSE ~ 0)) 

(ggplot()+
  geom_tile(data = g2, aes(x=x, y=y, fill = quality))+
  scale_fill_gradient(low = "#c2e9cf", high = "#046104") +
  geom_point(data=g3,aes(x=x, y=y), color = "#003cff")+
  geom_point(data= i1,aes(x=x, y=y, color = 'red'), size = 1)+#, position=position_dodge(width=1))+
  scale_x_reverse()+
  coord_flip()
)

ggplot(globals)+
geom_line(aes(iteration, n_individuals))


#-------------- HR tester decay time

hr_raw <- read.csv("./output/Hr_experiment experiment1-table.csv", header = TRUE, skip = 6)

#hr_raw <- read.csv("./output/Hr_experiment experiment1.csv", header = TRUE, skip = 6)

hr_sum <- hr_raw %>% group_by(decayTime, X.step.) %>% filter(X.step. > 2500) %>% summarise(meanHr = mean(cellsThatAreHr), meanAP = mean(nAp))

(hr_dec <- ggplot(hr_sum)+
geom_smooth(aes(X.step., meanHr, color = as.factor(decayTime)))+
geom_line(aes(X.step., meanHr, color = as.factor(decayTime)), alpha = .4)+
theme_bw()+
scale_color_scico_d(palette = "berlin", name = "Cell decay time")+
labs(y = "Mean homerange size", x = "Time")+
 theme(
    axis.text = element_text(size = 26),
    axis.title = element_text(size = 28),
    legend.text = element_text(size = 26),
    legend.title = element_text(size = 28),
    strip.text.x = element_text(size = 26),
    strip.text.y = element_text(size = 26),
    legend.spacing.y = unit(.25, 'cm')
  ) 
)


 p_name1 <-
    paste("./output/",
          "hr_decay",
          ".png",
          sep = "")
  
  ggsave(
    p_name1,
    plot = hr_dec,
    dpi = 300,
    limitsize = TRUE,
    width = 18,
    height = 12
  )





(ap_dec <- ggplot(hr_sum)+
geom_smooth(aes(X.step., meanAP, color = as.factor(decayTime)))+
geom_line(aes(X.step., meanAP, color = as.factor(decayTime)), alpha = .4)+
theme_bw()+
scale_color_scico_d(palette = "berlin", name = "Cell decay time")+
labs(y = "Mean n attraction points", x = "Time")+
 theme(
    axis.text = element_text(size = 26),
    axis.title = element_text(size = 28),
    legend.text = element_text(size = 26),
    legend.title = element_text(size = 28),
    strip.text.x = element_text(size = 26),
    strip.text.y = element_text(size = 26),
    legend.spacing.y = unit(.25, 'cm')
  ) 
)

 p_name2 <-
    paste("./output/",
          "ap_decay",
          ".png",
          sep = "")
  
  ggsave(
    p_name2,
    plot = ap_dec,
    dpi = 300,
    limitsize = TRUE,
    width = 18,
    height = 12
  )

#(ggplot(hr_raw)+
#geom_smooth(aes(X.step.,cellsThatAreHr , color = as.factor(decayTime)))
#
#)

#-------------------


#cellInfo <- read.csv("./output/debugCellinfo.csv", header = T)
#
#(ci1<-ggplot(cellInfo)+
#  geom_tile(aes(x=x_grid_o, y=y_grid_o, fill = quality))+
#  scale_fill_gradient(low = "#c2e9cf", high = "#046104")# +
# # geom_point(data= i1,aes(x=x, y=y, color = 'red'), size = 5, position=position_dodge(width=1))
#)
#g3 <- g2 %>% mutate(y = max(y) - y)
#
#(p1<-ggplot(g3)+
#  geom_tile(aes(x=x, y=y, fill = quality))+
#  scale_fill_gradient(low = "#d1dde0", high = "#313232") +
#  geom_point(data= i1,aes(x=x, y=y, color = 'red'), size = 5, position=position_dodge(width=1))
#)
#
#g4 <- g2 %>%
#mutate(nx = y, ny = max(x)-x)
#(p1<-ggplot(g4)+
#  geom_tile(aes(x=nx, y=ny, fill = quality))+
#  scale_fill_gradient(low = "#d1dde0", high = "#313232") +
#  geom_point(data= i1,aes(x=x, y=y, color = 'red'), size = 5, position=position_dodge(width=1))
#)
#
#head(g4)

(p2<-ggplot(individuals)+
  geom_line(aes(iteration,age, color= as.factor(id)))
)


r1 <- raster("./input/landscape/redDeer_global_50m.asc")

plot(r1)


SurvAA <- 0.65
SurvAP <- 0.50

1- (SurvAA)^(1/365)
1- (SurvAP)^(1/365)

1-(SurvAA^(1/12))
1-(SurvAP^(1/12))


(SurvAA^(1/12))

(SurvAP^(1/12))