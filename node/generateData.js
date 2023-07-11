import { PrismaClient } from '@prisma/client';
import { faker } from '@faker-js/faker';
const prisma = new PrismaClient();
import { spawn } from 'node:child_process';
async function main() {
	const name = () => faker.random.alphaNumeric(10000);
	const arr = [];
	for (let index = 0; index < 10000; index++) {
		arr.push(
			prisma.user2.create({
				data: {
					name: faker.name.fullName(),
					email: faker.internet.email(),
					name2: name(),
					name1: name(),
					name3: name(),
					name4: name(),
					name5: name(),
					name6: name(),
					name7: name(),
					name8: name(),
					name9: name(),
					name10: name(),
					name11: name(),
					name12: name(),
					name13: name(),
					name14: name(),
					name15: name(),
					name16: name(),
					name17: name(),
					name18: name(),
					name19: name(),
					name20: name(),
					name21: name(),
					name22: name(),
					name23: name(),
					name24: name(),
					name25: name(),
					name26: name(),
					name27: name(),
					name28: name(),
					name29: name(),
					name30: name(),
					name31: name(),
					name32: name(),
					name33: name(),
					name34: name(),
					name35: name(),
					name36: name(),
					name37: name(),
					name38: name(),
					name39: name(),
					name40: name(),
					name41: name(),
					name42: name(),
					name43: name(),
					name44: name(),
					name45: name(),
					name46: name(),
					name47: name(),
					name48: name(),
					name49: name(),
					name50: name(),
					name51: name(),
					name52: name(),
					name53: name(),
					name54: name(),
					name55: name(),
					name56: name(),
					name57: name(),
					name58: name(),
					name59: name(),
					name60: name(),
					name61: name(),
					name62: name(),
					name63: name(),
					name64: name(),
					name65: name(),
					name66: name(),
					name67: name(),
					name68: name(),
					name69: name(),
					name70: name(),
					name71: name(),
					name72: name(),
					name73: name(),
					name74: name(),
					name75: name(),
					name76: name(),
					name77: name(),
					name78: name(),
					name79: name(),
					name80: name(),
					name81: name(),
					name82: name(),
					name83: name(),
					name84: name(),
					name85: name(),
					name86: name(),
					name87: name(),
					name88: name(),
					name89: name(),
					name90: name(),
					name91: name(),
					name92: name(),
					name93: name(),
					name94: name(),
					name95: name(),
					name96: name(),
					name97: name(),
					name98: name(),
					name99: name(),
					name100: name(),
					name101: name(),
					name102: name(),
					name103: name(),
					name104: name(),
					name105: name(),
					name106: name(),
					name107: name(),
					name108: name(),
					name109: name(),
					name110: name(),
					name111: name(),
					name112: name(),
					name113: name(),
					name114: name(),
					name115: name(),
					name116: name(),
					name117: name(),
					name118: name(),
					name119: name(),
					name120: name()
				}
			})
		);
		if (index % 100 === 0) {
			await Promise.all(arr);
			arr.length = 0;
			console.log(index);
		}
	}
	const a = await prisma.user2.findFirst();
	console.log(a);
}

main()
	.then(async () => {
		await prisma.$disconnect();
	})
	.catch(async (e) => {
		console.error(e);
		await prisma.$disconnect();
		process.exit(1);
	});
