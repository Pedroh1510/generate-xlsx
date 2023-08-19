package main

import (
	"database/sql"
	"fmt"
	"log"
	"reflect"

	"github.com/go-sql-driver/mysql"
	"github.com/xuri/excelize/v2"
)

var db *sql.DB

type User struct {
	ID      uint32
	Email   string
	Name    string
	Name1   string
	Name2   string
	Name3   string
	Name4   string
	Name5   string
	Name6   string
	Name7   string
	Name8   string
	Name9   string
	Name10  string
	Name11  string
	Name12  string
	Name13  string
	Name14  string
	Name15  string
	Name16  string
	Name17  string
	Name18  string
	Name19  string
	Name20  string
	Name21  string
	Name22  string
	Name23  string
	Name24  string
	Name25  string
	Name26  string
	Name27  string
	Name28  string
	Name29  string
	Name30  string
	Name31  string
	Name32  string
	Name33  string
	Name34  string
	Name35  string
	Name36  string
	Name37  string
	Name38  string
	Name39  string
	Name40  string
	Name41  string
	Name42  string
	Name43  string
	Name44  string
	Name45  string
	Name46  string
	Name47  string
	Name48  string
	Name49  string
	Name50  string
	Name51  string
	Name52  string
	Name53  string
	Name54  string
	Name55  string
	Name56  string
	Name57  string
	Name58  string
	Name59  string
	Name60  string
	Name61  string
	Name62  string
	Name63  string
	Name64  string
	Name65  string
	Name66  string
	Name67  string
	Name68  string
	Name69  string
	Name70  string
	Name71  string
	Name72  string
	Name73  string
	Name74  string
	Name75  string
	Name76  string
	Name77  string
	Name78  string
	Name79  string
	Name80  string
	Name81  string
	Name82  string
	Name83  string
	Name84  string
	Name85  string
	Name86  string
	Name87  string
	Name88  string
	Name89  string
	Name90  string
	Name91  string
	Name92  string
	Name93  string
	Name94  string
	Name95  string
	Name96  string
	Name97  string
	Name98  string
	Name99  string
	Name100 string
	Name101 string
	Name102 string
	Name103 string
	Name104 string
	Name105 string
	Name106 string
	Name107 string
	Name108 string
	Name109 string
	Name110 string
	Name111 string
	Name112 string
	Name113 string
	Name114 string
	Name115 string
	Name116 string
	Name117 string
	Name118 string
	Name119 string
	Name120 string
}

const PAGE_SIZE = 100

func main() {
	cfg := mysql.Config{
		User:   "root",
		Passwd: "mauFJcuf5dhRMQrjj",
		Net:    "tcp",
		Addr:   "127.0.0.1:3306",
		DBName: "base",
	}
	var err error
	db, err = sql.Open("mysql", cfg.FormatDSN())
	if err != nil {
		log.Fatal(err)
	}
	pingErr := db.Ping()
	if pingErr != nil {
		log.Fatal(pingErr)
	}
	fmt.Println("Connected!")

	xlsx_write()
	fmt.Println("Finished!")
}

func xlsx_write() {
	f := excelize.NewFile()
	defer func() {
		if err := f.Close(); err != nil {
			fmt.Println(err)
		}
	}()
	sw, err := f.NewStreamWriter("Sheet1")
	if err != nil {
		fmt.Println(err)
		return
	}
	for offset := 0; offset < 5000; offset += PAGE_SIZE {
		println(offset)
		var users, er = listUsers2(uint32(offset))
		if er != nil {
			log.Fatal(er)
			break
		}
		for i, details := range users {
			v := reflect.ValueOf(details)

			row := make([]interface{}, 200)
			for i := 0; i < v.NumField(); i++ {
				row[i] = v.Field(i).Interface()
			}

			cell, err := excelize.CoordinatesToCellName(1, offset+i+1)
			if err != nil {
				fmt.Println(err)
				break
			}
			if err := sw.SetRow(cell, row); err != nil {
				fmt.Println(err)
				break
			}

		}
	}

	if err := sw.Flush(); err != nil {
		fmt.Println(err)
		return
	}
	if err := f.SaveAs("Book1.xlsx"); err != nil {
		fmt.Println(err)
	}
}

func listUsers2(page uint32) ([]User, error) {
	var users []User
	var query = fmt.Sprintf("SELECT * FROM User2 LIMIT %d OFFSET %d", PAGE_SIZE, page)
	rows, err := db.Query(query)
	if err != nil {
		return nil, fmt.Errorf("%v", err)
	}
	defer rows.Close()
	// var users [][]string
	for rows.Next() {
		user := User{}

		s := reflect.ValueOf(&user).Elem()
		numCols := s.NumField()
		columns := make([]interface{}, numCols)
		for i := 0; i < numCols; i++ {
			field := s.Field(i)
			columns[i] = field.Addr().Interface()
		}

		err := rows.Scan(columns...)
		if err != nil {
			log.Fatal(err)
		}
		users = append(users, user)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("%v", err)
	}
	return users, err
}
